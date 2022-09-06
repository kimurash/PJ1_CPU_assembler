# PJ2_CPU_assembler
## 開発の動機
プロジェクト実習2のテーマ「CPU」では、
単純な8ビットコンピュータを搭載した教育用CPUボードの
命令実行機能をシミュレートするプログラムを開発する．

開発したプログラムの正当性を示すためには、
実装した命令機能に応じてテストプログラムを
用意しなければならない．実習テキストには、
各命令と命令語コード(16進数の羅列)との対応が
表形式で与えられているため、アセンブリ言語で
書いたプログラムをハンドアセンブルできる．

しかし、「表を参照して命令語コードへと翻訳する」
という決まり切った作業は本来機械にやらせるべきである．
本テーマの実習期間中にアセンブラを開発する余裕を
確保できなかった私はハンドアセンブルする他なく、
自分の信条とは反する行動を取らなければならないことに、
如何ともしがたい違和感を覚えた．

当時のやるせない気持ちを晴らしたいという思いが、
アセンブラの開発に乗り出した動機の1つである．

私は本実習を終えてしまったため、この成果物が自身に
寄与することはないが、ハンドアセンブルの自動化を望む
後輩の役に立てるのであれば、これに勝る喜びはない．

## 開発の方針
アセンブラの開発は次の2段階の手順で行った．
1. 記号アドレスを含まないプログラムを変換する
   アセンブラを書く．
2. 記号アドレスを扱えるように先のアセンブラを
   拡張する．

アセンブリプログラムでは、**記号アドレス**と呼ばれるラベル
を用いることができる．これは分岐命令の分岐先として、実際の
アドレスの代わりに指定することができる．記号アドレスは、
それが定義される前であっても用いることができるため、
プログラムを前から逐次的にアセンブルしていく方法では
うまくいかない．未知のラベルに遭遇したとき、それを
実際のアドレスに変換する術がないからである．そこで、
一度プログラムを読み切り、ラベルと実際のアドレスの
対応表(以下、記号表と呼ぶ)を作成してから、アセンブルを
開始するという方針をとった．

## 実装
アセンブリは以下のモジュールで構成されている．

### `instruction`モジュール
- 独自に定義した型と
  それに実装したメソッドの定義が書いてある．

### `parser`モジュール
- アセンブリプログラムを読みながら各命令を独自の
  `Instruction`構造体へと変換し、ベクタへと格納する．
- ラベルが定義されていると、
  アドレスとの対応を記号表に登録する．

### `code`モジュール
- `parser`モジュールが返すベクタを走査して、
  各命令を命令語コードへと変換する．

## 仕様
### アセンブリ言語
実習テキスト表1の形式に則ったアセンブリ言語を変換の対象とする．
ただし、`ST`命令の第2オペランドには絶対アドレスかインデックス
修飾アドレスしか許されていないが、これに対するエラーチェックは
行わない．

### 命令語コード
実習テキスト表5に示された命令語コードに変換する．

## 動作
### 実行環境
このアセンブラはRustで書かれているため、
Rustの実行環境がないと動作しない．

Rustのプログラムをコンパイルしてバイナリを生成するためには
以下のソフトウェアが必要となる．
1. **Rustツールチェイン**
  - Rustで書かれたソースコードをコンパイルするために
    使われるプログラミングツールの総称．以下のもので
    構成されている．

    - `rustc`コマンド: Rustコンパイラ
    - `cargo`コマンド: Rustのビルドマネージャ兼パッケージマネージャ
    - `std`: Rustの標準ライブラリ
   
  - Rustプロジェクトが公式にサポートしているコマンドラインツール
    `rustup`でインストールできる．`rustup`には以下のような機能がある．

    - Rustツールチェインの複数バージョンのインストールと管理
    - クロスコンパイル用ターゲットのインストール
    - RLSなどの開発支援ツールのインストール  
      (RLS: Rust Language Server)

2. ターゲット環境向けのリンカ
   - Linux: gcc, binutilsパッケージ
   - MacOS: Xcodeのコマンドライン・デベロッパツール
   - Windows MSVC: Microsoft Visual C++ビルドツール

### 実行方法
1. 変換したいアセンブリプログラムを用意する．
2. `main.rs`の`parse()`関数の第1引数に、
   上記ファイルへのパスを文字列で与える．
3. `main.rs`の`assemble()`関数の第1引数に、
   出力ファイルへのパスを文字列で与える．
4. `cargo run`を叩いて実行．

2.3.で入/出力ファイルへのパスを相対パスで与える際は、
4.で`cargo run`を叩く位置からの相対パスを記述する．

### 実行例
実習テキスト図3に示されているサンプルプログラムを
アセンブルした結果を例として示そうと思ったが、
これを示したところで信じるか否かという客観的な話
にしかならないので取り止めた．

## 参考文献
- *k*een, 河野達也, 小松礼人．実践Rust入門．
  初版第2刷．技術評論社, 2020年, 551p．
