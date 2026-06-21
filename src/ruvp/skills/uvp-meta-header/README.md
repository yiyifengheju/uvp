# UVP Meta Header Skill - 配置指南

## IDE 配置

### Trae IDE（推荐）
自动识别 `~/.uvp/skills/` 目录，无需额外配置。

### Cursor IDE
Settings → Agents → Custom Instructions，添加：
```markdown
When creating/editing any `.md` under `docs/`, read and follow `~/.uvp/skills/uvp-meta-header/SKILL.md`
```

### Claude Code
在 `CLAUDE.md` 顶部添加：
```markdown
> When creating files in `docs/`, invoke uvp-meta-header skill: `~/.uvp/skills/uvp-meta-header/SKILL.md`
```

### VS Code + Continue Dev
`.continue/config.yaml`:
```yaml
skills:
  - path: ~/.uvp/skills/uvp-meta-header
    trigger:
      - "create.*docs/"
      - "write.*docs/"
```

---

## 验证

```bash
uvp check    # 第 6 项检查 meta 头一致性
```

---

## 故障排查

| 问题 | 解决 |
|------|------|
| AI 没加 meta 头 | 确认 `~/.uvp/skills/uvp-meta-header/SKILL.md` 存在，手动提及 skill |
| 格式错误 | 运行 `uvp check` 查看具体错误 |
| 缺少 title/date | 添加 `title: "..."` 和 `date: "YYYY-MM-DD"` |
