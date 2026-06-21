# FEAT-014 Verification

## 验证状态

✅ 已验证

## 验收标准

- [x] `uvp init` 创建 `docs/TODO.md`
- [x] `uvp todo` 正确列出待办项
- [x] `uvp todo add "内容"` 正确添加条目
- [x] `uvp todo done 1` 正确标记完成并移动
- [x] `uvp todo remove 1` 正确删除条目
- [x] `uvp todo --all` 显示含已完成条目
- [x] ID 自增正确
- [x] 文件不存在时自动创建

## 验证结果

| 检查项 | 结果 | 说明 |
|--------|------|------|
| todo list | ✅ | 空列表显示"暂无 TODO"，有数据显示编号+内容+日期 |
| todo add | ✅ | 自动分配 ID，追加到待办区 |
| todo done | ✅ | 标记完成，移到已完成区，记录完成日期 |
| todo remove | ✅ | 正确删除，不存在时提示错误 |
| --all | ✅ | 同时显示待办和已完成 |
| 自动创建 | ✅ | TODO.md 不存在时自动创建模板 |
| init 集成 | ✅ | write_file_safe 创建 docs/TODO.md |
