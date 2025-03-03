# Personal Information Scanner

ファイル内の個人情報（メールアドレス、電話番号、クレジットカード番号など）を検出するスキャナーツールです。

## 概要

このツールは、指定されたディレクトリ内のファイル（テキストファイル、PDFファイル、DOCXファイル）をスキャンし、個人情報が含まれているかどうかを判定します。個人情報の検出には、Ollama APIを通じてDeepseek Coderなどの大規模言語モデル（LLM）を利用するか、またはフォールバックとして正規表現ベースの検出を使用します。

## 機能

- ディレクトリの再帰的スキャン
- 単一ファイルのスキャン
- PDFファイルからのテキスト抽出と分析
- DOCXファイルからのテキスト抽出と分析
- 複数の個人情報タイプの検出:
  - メールアドレス
  - 電話番号
  - クレジットカード番号
  - その他（Ollama APIを使用する場合）
- API使用またはローカル分析の選択
- JSON形式での結果出力

## インストール

### 要件

- Rust 2021 Edition以上
- Ollama（APIを使用する場合）

### ビルド方法

```bash
# リポジトリをクローン
git clone https://github.com/yourusername/personal-info-scanner.git
cd personal-info-scanner

# ビルド
cargo build --release

# インストール (オプション)
cargo install --path .
```

## 使い方

### 基本的な使い方

```bash
# ディレクトリをスキャン
personal-info-scanner scan /path/to/directory

# 単一ファイルをスキャン
personal-info-scanner scan-file /path/to/file.txt
```

### オプション

#### ディレクトリスキャン (`scan`)

```bash
# すべてのオプションを表示
personal-info-scanner scan --help

# APIを使わずに正規表現のみでスキャン
personal-info-scanner scan /path/to/directory --no-api

# PDF処理を無効化してスキャン
personal-info-scanner scan /path/to/directory --pdf false

# DOCX処理を無効化してスキャン
personal-info-scanner scan /path/to/directory --docx false

# 結果をファイルに出力
personal-info-scanner scan /path/to/directory --output results.json

# 詳細なログを表示
personal-info-scanner scan /path/to/directory --verbose

# カスタムAPIエンドポイントとモデルを指定
personal-info-scanner scan /path/to/directory --api-url http://localhost:11434/api/generate --model llama3
```

#### ファイルスキャン (`scan-file`)

```bash
# APIを使わずに正規表現のみでスキャン
personal-info-scanner scan-file /path/to/file.txt --no-api

# 結果をファイルに出力
personal-info-scanner scan-file /path/to/file.txt --output result.json

# 詳細なログを表示
personal-info-scanner scan-file /path/to/file.txt --verbose
```

## Ollama APIの設定

このツールはデフォルトで `http://localhost:11434/api/generate` のOllama APIエンドポイントに接続を試みます。APIを使用するには：

1. [Ollama](https://ollama.ai/)をインストールして実行
2. Deepseek Coderまたは同等のモデルを取得: `ollama pull deepseek-coder`

## 出力形式

出力は以下のようなJSON形式です:

```json
[
  {
    "file": "/path/to/file1.txt",
    "personal_information": [
      {
        "type_": "email",
        "value": "test@example.com",
        "line": 1,
        "start": 24,
        "end": 40
      },
      {
        "type_": "phone_number",
        "value": "090-1234-5678",
        "line": 5,
        "start": 14,
        "end": 27
      }
    ]
  },
  {
    "file": "/path/to/file2.txt",
    "personal_information": []
  }
]
```

各フィールドの意味:
- `file`: スキャンされたファイルのパス
- `personal_information`: 検出された個人情報の配列
  - `type_`: 個人情報の種類（email, phone_number, credit_card など）
  - `value`: 検出された個人情報の値
  - `line`: 個人情報が見つかった行番号
  - `start`: 行内での開始位置（文字インデックス）
  - `end`: 行内での終了位置（文字インデックス）

## 制限事項

- PDFファイルの複雑なレイアウトでは、テキスト抽出の精度が低下する場合があります
- 暗号化されたPDFファイルは処理できません
- API呼び出しを使用しない場合（`--no-api`）、正規表現ベースの検出は限られた種類の個人情報のみを検出します
- 非テキストファイル（画像など）の内容は解析されません

## ライセンス

MIT

## 貢献

バグ報告や機能追加の提案は、Issueまたはプルリクエストを通じてお願いします。
