# RuleZ Copilot hook wrapper — forwards stdin to cch copilot hook
$input = [Console]::In.ReadToEnd()
$input | cch copilot hook
