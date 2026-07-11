#!/usr/bin/env bash
# cplib の #[snippet] 注釈を抽出し、Neovim(neosnippet) 用スニペットを生成する。
#
# 使い方: src/ のスニペットを編集したら本スクリプトを実行して snippets/ を更新する。
#   ./gen-snippets.sh
#
# neosnippet.vim ならこの snippets/ ディレクトリを g:neosnippet#snippets_directory に、
# LuaSnip(from_vscode) 派は下部のコメントの vscode 形式を使う。
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p snippets

cargo snippet -t neosnippet > snippets/cplib.snip
echo "wrote snippets/cplib.snip (neosnippet)"

# LuaSnip の from_vscode ローダや VSCode 本体を使う場合はこちらも生成する:
# cargo snippet -t vscode > snippets/cplib.code-snippets
