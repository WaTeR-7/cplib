// スニペット原本なのでクレート内から呼ばれない。modint は std のトレイトを実装している
// ぶん生存扱いになるが、こちらは素の struct と関数なので dead_code が出る。
// この属性はファイル（= pub mod dfs）に掛かるだけで、#[snippet] が抽出する
// mod my_template_dfs の中には入らないため、貼り付け先には複製されない。
#![allow(dead_code)]

use cargo_snippet::snippet;

/// 再帰 DFS の雛形。**貼ってから問題に合わせて書き換える**前提のスケルトン。
///
/// 書き換えるのは次の 3 箇所だけ。
///
/// 1. `Dfs` に、探索中に**読むだけ**のデータを足す（重み、コスト、前計算テーブルなど）
/// 2. `DfsTable` に、探索中に**書き換える**データを足す（メモ、部分木の集計など）
/// 3. `call` の戻り値の型と本体を書く
///
/// 読むものと書くものを別の型に分けているのが要点。ぜんぶ一つの構造体に入れて
/// `&mut self` のメソッドにすると、`&self.graph[v]` を回しながら `self.call()` を
/// 呼べなくなる（`cannot borrow *self as mutable because it is also borrowed as
/// immutable`、E0502）。`&self` と `&mut DfsTable` に分ければ借用が衝突しない。
///
/// ネストした `fn` に全部引数で渡す書き方でも同じことはできるが、読むデータが
/// 3 つ 4 つに増えると再帰呼び出しの行が引数で埋まる。この形なら `self.call(u, dt)`
/// のまま変わらない。
///
/// 再帰の深さは頂点数に比例しうる。解答テンプレートの `fn main` が 256MB の
/// スタックを確保したスレッドで `solve` を呼ぶので、20 万頂点のパスでも落ちない。
/// テンプレート以外の場所に貼るなら、スタックを自分で確保すること。
#[snippet("dfs")]
#[snippet(prefix = "use my_template_dfs::*;")]
mod my_template_dfs {
    /// DFS 中に読み取るだけのデータ。問題ごとにフィールドを足す。
    pub struct Dfs {
        pub graph: Vec<Vec<usize>>,
    }

    /// DFS 中に書き換えるデータ。問題ごとにフィールドを足す。
    pub struct DfsTable {
        pub seen: Vec<bool>,
    }

    impl Dfs {
        /// `graph` の頂点数に合わせた初期状態を作る。フィールドを足したらここも直す。
        pub fn table(&self) -> DfsTable {
            DfsTable {
                seen: vec![false; self.graph.len()],
            }
        }

        /// `s` から辿れる頂点数を返す。**戻り値の型と本体を問題に合わせて書き換える。**
        pub fn call(&self, s: usize, dt: &mut DfsTable) -> u64 {
            dt.seen[s] = true;
            let mut acc = 1;
            for &d in &self.graph[s] {
                if !dt.seen[d] {
                    acc += self.call(d, dt);
                }
            }
            acc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::my_template_dfs::*;

    /// 決定的な xorshift。テストの再現性のため。
    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
    }

    fn undirected(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut g = vec![vec![]; n];
        for &(x, y) in edges {
            g[x].push(y);
            g[y].push(x);
        }
        g
    }

    #[test]
    fn counts_the_reachable_vertices() {
        // 0-1, 0-2, 1-3, 2-3 は閉路。3-4 がぶら下がり、5 は孤立
        let g = undirected(6, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)]);
        let dfs = Dfs { graph: g };
        let mut dt = dfs.table();
        assert_eq!(dfs.call(0, &mut dt), 5);
        assert!(!dt.seen[5], "孤立点は訪問しない");
    }

    #[test]
    fn marks_exactly_the_visited_vertices() {
        let g = undirected(6, &[(0, 1), (0, 2), (1, 3), (2, 3), (3, 4)]);
        let dfs = Dfs { graph: g };
        let mut dt = dfs.table();
        dfs.call(0, &mut dt);
        assert_eq!(dt.seen, vec![true, true, true, true, true, false]);
    }

    #[test]
    fn single_vertex() {
        let dfs = Dfs {
            graph: vec![vec![]],
        };
        let mut dt = dfs.table();
        assert_eq!(dfs.call(0, &mut dt), 1);
    }

    #[test]
    fn self_loop_does_not_hang() {
        // 自己ループは seen で弾かれる
        let mut g = vec![vec![]; 2];
        g[0].push(0);
        g[0].push(1);
        g[1].push(0);
        let dfs = Dfs { graph: g };
        let mut dt = dfs.table();
        assert_eq!(dfs.call(0, &mut dt), 2);
    }

    #[test]
    fn table_can_be_reused_across_components() {
        // 0-1 と 2-3 と 4 の 3 成分
        let g = undirected(5, &[(0, 1), (2, 3)]);
        let dfs = Dfs { graph: g };
        let mut dt = dfs.table();
        let mut sizes = vec![];
        for v in 0..5 {
            if !dt.seen[v] {
                sizes.push(dfs.call(v, &mut dt));
            }
        }
        assert_eq!(sizes, vec![2, 2, 1]);
        assert!(dt.seen.iter().all(|&s| s));
    }

    #[test]
    fn matches_a_brute_force_reachability() {
        let mut rng = Rng(20260831);
        for _ in 0..500 {
            let n = rng.below(10) + 1;
            let m = rng.below(15);
            let edges: Vec<(usize, usize)> = (0..m).map(|_| (rng.below(n), rng.below(n))).collect();
            let g = undirected(n, &edges);

            // 素朴な到達可能集合（変化がなくなるまで緩和するだけ）
            let mut reach = vec![false; n];
            reach[0] = true;
            loop {
                let mut changed = false;
                for v in 0..n {
                    if reach[v] {
                        for &u in &g[v] {
                            if !reach[u] {
                                reach[u] = true;
                                changed = true;
                            }
                        }
                    }
                }
                if !changed {
                    break;
                }
            }

            let dfs = Dfs { graph: g };
            let mut dt = dfs.table();
            let got = dfs.call(0, &mut dt);
            assert_eq!(dt.seen, reach);
            assert_eq!(got, reach.iter().filter(|&&r| r).count() as u64);
        }
    }

    #[test]
    fn survives_a_deep_path_on_the_template_stack() {
        // 解答テンプレートと同じ 256MB のスタックで走らせる。
        // cargo test の既定スタックは 2MB なので、ここで確保しないと落ちる。
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(|| {
                let n = 200_000;
                let edges: Vec<(usize, usize)> = (1..n).map(|v| (v - 1, v)).collect();
                let dfs = Dfs {
                    graph: undirected(n, &edges),
                };
                let mut dt = dfs.table();
                assert_eq!(dfs.call(0, &mut dt), n as u64);
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
