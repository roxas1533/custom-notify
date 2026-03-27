# Custom Notify

Windows 向けカスタム通知表示アプリ。HTTP API 経由で通知を送信し、画面上にカスタムデザインの通知バナーを表示します。

## インストール

[Releases](https://github.com/roxas1533/custom-notify/releases) から最新の `Custom.Notify_x.x.x_x64-setup.exe` をダウンロードして実行してください。

## 使い方

起動するとタスクトレイに常駐します。HTTP API で通知を送信できます。

### 通知を送信する

```bash
curl -X POST http://localhost:19090/notify \
  -H "Content-Type: application/json" \
  -d '{"title": "Hello", "body": "This is a notification"}'
```

### リクエストボディ

| フィールド | 型 | 必須 | 説明 |
|---|---|---|---|
| `title` | string | Yes | 通知タイトル |
| `body` | string | Yes | 通知本文 |
| `style` | string | No | `info` / `success` / `warning` / `error` (デフォルト: `info`) |
| `duration_ms` | number | No | 表示時間 (ミリ秒)。`0` で手動で閉じるまで表示 |
| `icon_url` | string | No | アイコン画像 URL |

### 通知スタイルの例

```bash
# 成功
curl -X POST http://localhost:19090/notify \
  -H "Content-Type: application/json" \
  -d '{"title": "Deploy", "body": "Deployment succeeded", "style": "success"}'

# エラー
curl -X POST http://localhost:19090/notify \
  -H "Content-Type: application/json" \
  -d '{"title": "Error", "body": "Build failed", "style": "error"}'

# 警告 (消えない通知)
curl -X POST http://localhost:19090/notify \
  -H "Content-Type: application/json" \
  -d '{"title": "Warning", "body": "Disk usage over 90%", "style": "warning", "duration_ms": 0}'
```

### ヘルスチェック

```bash
curl http://localhost:19090/health
# => OK
```

## 設定

タスクトレイアイコンを右クリック → 「設定」から GUI で変更できます。

設定ファイル: `%APPDATA%\custom-notify\settings.toml`

```toml
port = 19090
notification_position = "bottom_right"  # top_right / top_left / bottom_right / bottom_left
notification_duration_ms = 5000
max_visible_notifications = 5
notification_width = 360
notification_height = 120
notification_gap = 8
animation_duration_ms = 300
```

## 開発

### 必要なもの

- [Bun](https://bun.sh/)
- [Rust](https://rustup.rs/)

### ローカルビルド (WSL → Windows クロスコンパイル)

```bash
bun install
bun run build
cd src-tauri && cargo xwin build --target x86_64-pc-windows-msvc --release
```

### リリース

タグを push すると GitHub Actions で Windows インストーラが自動ビルドされます。

```bash
git tag v0.1.0
git push origin v0.1.0
```
