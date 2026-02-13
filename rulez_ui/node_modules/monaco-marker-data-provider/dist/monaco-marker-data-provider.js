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
export function registerMarkerDataProvider(monaco, languageSelector, provider) {
    const listeners = new Map();
    const matchesLanguage = (languageId) => {
        if (languageSelector === '*') {
            return true;
        }
        return Array.isArray(languageSelector)
            ? languageSelector.includes(languageId)
            : languageSelector === languageId;
    };
    const doValidate = async (model) => {
        const versionId = model.getVersionId();
        const markers = await provider.provideMarkerData(model);
        // The model may have been updated disposed by the time marker data has been fetched.
        if (!model.isDisposed() &&
            versionId === model.getVersionId() &&
            matchesLanguage(model.getLanguageId())) {
            monaco.editor.setModelMarkers(model, provider.owner, markers ?? []);
        }
    };
    const onModelAdd = (model) => {
        if (!matchesLanguage(model.getLanguageId())) {
            return;
        }
        let handle;
        const onDidChangeContent = model.onDidChangeContent(() => {
            clearTimeout(handle);
            handle = setTimeout(() => {
                doValidate(model);
            }, 500);
        });
        listeners.set(model, {
            dispose() {
                clearTimeout(handle);
                onDidChangeContent.dispose();
            }
        });
        doValidate(model);
    };
    const onModelRemoved = (model) => {
        monaco.editor.setModelMarkers(model, provider.owner, []);
        const listener = listeners.get(model);
        if (listener) {
            listener.dispose();
            listeners.delete(model);
        }
    };
    /**
     * Call `doReset` on the provider if the model language matches.
     *
     * @param model
     *   The model to reset
     * @param languageId
     *   The language ID to check.
     */
    function doReset(model, languageId = model.getLanguageId()) {
        if (provider.doReset && matchesLanguage(languageId)) {
            provider.doReset(model);
        }
    }
    const onDidCreateModel = monaco.editor.onDidCreateModel(onModelAdd);
    const onWillDisposeModel = monaco.editor.onWillDisposeModel((model) => {
        onModelRemoved(model);
        doReset(model);
    });
    const onDidChangeModelLanguage = monaco.editor.onDidChangeModelLanguage(({ model, oldLanguage }) => {
        onModelRemoved(model);
        onModelAdd(model);
        doReset(model, oldLanguage);
    });
    for (const model of monaco.editor.getModels()) {
        onModelAdd(model);
    }
    return {
        dispose() {
            for (const model of listeners.keys()) {
                onModelRemoved(model);
            }
            onDidCreateModel.dispose();
            onWillDisposeModel.dispose();
            onDidChangeModelLanguage.dispose();
        },
        async revalidate() {
            await Promise.all(monaco.editor.getModels().map(doValidate));
        }
    };
}
//# sourceMappingURL=monaco-marker-data-provider.js.map