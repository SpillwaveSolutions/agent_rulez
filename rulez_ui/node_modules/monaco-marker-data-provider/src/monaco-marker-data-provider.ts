import type { editor, IDisposable, languages, MonacoEditor } from 'monaco-types'

export interface MarkerDataProvider {
  /**
   * The owner of the model markers.
   *
   * This should be a unique string that identifies the context of who owns the marker data.
   */
  owner: string

  /**
   * Provide marker data for the given model.
   *
   * @param model
   *   The model to provide marker data for.
   * @returns
   *   The new marker data for the model.
   */
  provideMarkerData: (
    model: editor.ITextModel
  ) => languages.ProviderResult<editor.IMarkerData[] | void>

  /**
   * Reset the state for a model.
   *
   * @param model
   *   The model to reset the state for.
   */
  doReset?: (model: editor.ITextModel) => unknown
}

export interface MarkerDataProviderInstance extends IDisposable {
  /**
   * Revalidate all models.
   */
  revalidate: () => Promise<undefined>
}

/**
 * Register a marker data provider that can provide marker data for a model.
 *
 * @param monaco
 *   The Monaco editor module.
 * @param languageSelector
 *   The language id to register the provider for.
 * @param provider
 *   The provider that can provide marker data.
 * @returns
 *   A disposable.
 */
export function registerMarkerDataProvider(
  monaco: Pick<MonacoEditor, 'editor'>,
  languageSelector: string | string[],
  provider: MarkerDataProvider
): MarkerDataProviderInstance {
  const listeners = new Map<editor.ITextModel, IDisposable>()

  const matchesLanguage = (languageId: string): boolean => {
    if (languageSelector === '*') {
      return true
    }

    return Array.isArray(languageSelector)
      ? languageSelector.includes(languageId)
      : languageSelector === languageId
  }

  const doValidate = async (model: editor.ITextModel): Promise<undefined> => {
    const versionId = model.getVersionId()
    const markers = await provider.provideMarkerData(model)
    // The model may have been updated disposed by the time marker data has been fetched.
    if (
      !model.isDisposed() &&
      versionId === model.getVersionId() &&
      matchesLanguage(model.getLanguageId())
    ) {
      monaco.editor.setModelMarkers(model, provider.owner, markers ?? [])
    }
  }

  const onModelAdd = (model: editor.ITextModel): undefined => {
    if (!matchesLanguage(model.getLanguageId())) {
      return
    }

    let handle: ReturnType<typeof setTimeout>
    const onDidChangeContent = model.onDidChangeContent(() => {
      clearTimeout(handle)
      handle = setTimeout(() => {
        doValidate(model)
      }, 500)
    })

    listeners.set(model, {
      dispose() {
        clearTimeout(handle)
        onDidChangeContent.dispose()
      }
    })

    doValidate(model)
  }

  const onModelRemoved = (model: editor.ITextModel): undefined => {
    monaco.editor.setModelMarkers(model, provider.owner, [])
    const listener = listeners.get(model)
    if (listener) {
      listener.dispose()
      listeners.delete(model)
    }
  }

  /**
   * Call `doReset` on the provider if the model language matches.
   *
   * @param model
   *   The model to reset
   * @param languageId
   *   The language ID to check.
   */
  function doReset(model: editor.ITextModel, languageId = model.getLanguageId()): undefined {
    if (provider.doReset && matchesLanguage(languageId)) {
      provider.doReset(model)
    }
  }

  const onDidCreateModel = monaco.editor.onDidCreateModel(onModelAdd)
  const onWillDisposeModel = monaco.editor.onWillDisposeModel((model) => {
    onModelRemoved(model)
    doReset(model)
  })
  const onDidChangeModelLanguage = monaco.editor.onDidChangeModelLanguage(
    ({ model, oldLanguage }) => {
      onModelRemoved(model)
      onModelAdd(model)
      doReset(model, oldLanguage)
    }
  )

  for (const model of monaco.editor.getModels()) {
    onModelAdd(model)
  }

  return {
    dispose() {
      for (const model of listeners.keys()) {
        onModelRemoved(model)
      }
      onDidCreateModel.dispose()
      onWillDisposeModel.dispose()
      onDidChangeModelLanguage.dispose()
    },

    async revalidate() {
      await Promise.all(monaco.editor.getModels().map(doValidate))
    }
  }
}
