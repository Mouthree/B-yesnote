# 文件图标

图标取自 [Phosphor Icons](https://phosphoricons.com/) 的 `regular` 变体，
版本 `@phosphor-icons/core@2.1.1`，许可证为 MIT（见同目录 `LICENSE`）。

- 统一为 256×256 viewBox 的线条风格，扁平、无阴影，适合深色界面。
- 全部图标都可以用 `svg::Style { color: ... }` 染色，颜色由代码决定。
- `src/view.rs` 通过 `include_bytes!` 把用到的图标嵌入二进制，运行时不需要 `assets` 目录。

## 文件类型

| 用途 | 文件 |
| --- | --- |
| Markdown | `file-md.svg` |
| 纯文本 | `file-text.svg` |
| PDF | `file-pdf.svg` |
| Word | `file-doc.svg` |
| Excel | `file-xls.svg` |
| PowerPoint | `file-ppt.svg` |
| 图片 | `file-image.svg` |
| 音频 | `file-audio.svg` |
| 视频 | `file-video.svg` |
| 压缩包 | `file-zip.svg` |
| CSV | `file-csv.svg` |
| 未知文件 | `file.svg` |

## 文件夹与笔记

`folder.svg`、`folder-open.svg`、`notebook.svg`、`note.svg`、`note-pencil.svg`、
`book-open.svg`、`bookmark-simple.svg`

## 界面

`magnifying-glass.svg`、`sidebar-simple.svg`、`plus.svg`、`gear-six.svg`、
`caret-down.svg`、`caret-right.svg`、`dots-three.svg`、`pencil-simple.svg`、
`trash.svg`、`star.svg`、`check.svg`

## 窗口按钮

`minus.svg`、`square.svg`、`x.svg`

## 新增图标

从以下地址下载，保持文件名不变：

```text
https://unpkg.com/@phosphor-icons/core@2.1.1/assets/regular/<图标名>.svg
```

图标名可以在 [phosphoricons.com](https://phosphoricons.com/) 上查到，通常是
PascalCase 转 kebab-case，例如 `FileMd` → `file-md.svg`。
