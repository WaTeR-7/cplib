#!/usr/bin/env bash
# cplib の #[snippet] 注釈を抽出し、VSCode 形式のスニペットを生成する。
#
# 使い方: src/ のスニペットを編集したら、出力先を指定して実行する。
#   ./gen-snippets.sh ~/.config/nvim/snippets/rust.json
#
# VSCode 形式は LuaSnip(from_vscode) / blink.cmp / VSCode 本体が読める。出力先は
# エディタ側の設定で決まるものなので、このスクリプトは既定値を持たない。
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "usage: $(basename "$0") <出力先の .json パス>" >&2
    echo "  例: $(basename "$0") ~/.config/nvim/snippets/rust.json" >&2
    exit 1
fi

# 相対パスは呼び出し元のカレントディレクトリ基準で解決してから cd する
out=$1
case "$out" in
    /*) ;;
    *) out="$PWD/$out" ;;
esac

cd "$(dirname "$0")"
mkdir -p "$(dirname "$out")"
cargo snippet -t vscode > "$out"
echo "wrote $out (vscode)"
