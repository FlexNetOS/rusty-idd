<!-- icm:start -->
## Persistent memory (ICM)

This project uses [ICM](https://github.com/rtk-ai/icm) for persistent memory across sessions.

### Recall (before starting work)
```bash
icm recall "topic keywords"              # search memories
icm recall-context "query" --limit 5     # formatted for prompt injection
```

### Store (after completing significant work)
```bash
icm store -t "topic" -c "summary" -i medium   # importance: critical|high|medium|low
icm store -t "decisions" -c "chose X over Y because..." -i high
icm store -t "errors-resolved" -c "fix: ..." -k "error,fix"
```

### Other commands
```bash
icm update <id> -c "updated content"     # edit memory in-place
icm health                                # topic hygiene audit
icm topics                                # list all topics
icm feedback record -t "topic" -c "context" -p "predicted" --corrected "actual"
```
<!-- icm:end -->
