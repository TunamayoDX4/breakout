# BreakOut

This is a simple breakout clone game by Atari.

There are still many bugs, but our first priority is to make sure that it works at a minimum and does not cause serious defects.

---

## Summary

I created this game because my mother asked me to make a game like Nintendo's block crusher, which I had played when I was a child.

I am also developing this software to learn programming and the Rust language, and to practice implementing multimedia processing.

---

## Special thanks

MaouDamashii([MaouDamashii](https://maou.audio/)) : Distributors of Free Sound Effects  
MPlusFonts([MPlufFonts](https://mplus-fonts.osdn.jp)) : Font Distributor

---

## License
MIT License([LICENSE-MIT](https://opensource.org/licenses/MIT)) or Apache License Version 2.0([LICENSE-APACHE](https://www.apache.org/licenses/LICENSE-2.0)) at your option.


---

…The following explanation is in Japanese, which is the native language of the author.

…以下は作者の母国語となる日本語による説明となります。

---

# ブロック崩し

シンプル・軽量なブロック崩しゲームです。

まだバグは多いですが、まずは最低限動くことと、重大な欠陥を生じないことを重視しております。

---

## 概要
母に小さい頃にやったことのある任天堂のブロック崩しが好きで、
そういった気軽にできるようなゲームを作ってほしいという要望があったこと、

プログラミングおよびRust言語の習熟、また興味のある分野であるマルチ・メディア処理の学習を
行いたいことも有って開発しているブロック崩しとなります。

変な挙動も多くお見苦しいプログラムではありますが、宜しくお願い申し上げます。

---

## 遊び方

いずれバイナリ、実行可能ファイルおよび周辺ファイルのみでの配布、という形を
考えておりますが、現状はソースコードでの配布とさせて頂き、

それにつきプレイするにあたってビルドをしていただく形となっております。
お手数をお掛けしてしまいますが、ご了承ください。

普通のGitHub上で公開されているリポジトリよろしくクローンおよびZip形式でダウンロードし、
[Rust言語の公式サイト](https://www.rust-lang.org/ja)の通りにRustコンパイラを
ご利用の環境ににインストールしてください。

インストールがお済みになれば、
クローンおよびzipファイルを展開したディレクトリをプロンプトやシェルで開き、  
『cargo build --release』  
と入力することで少し時間がかかりますが、クライアント側に問題がなければおそらく
コンパイルが通ります。

上手くいきましたら、  
"target/release"フォルダ内に吐き出される"breakout.exe"を、
クローン及び展開されたフォルダ、具体的にはfontおよびseフォルダが有るフォルダにコピー、  
もしくは適当なところに"breakout.exe"とfontおよびseフォルダを
全てコピーされることで実行が可能になります。


---

## スペシャル・サンクス

魔王魂([魔王魂](https://maou.audio/)) : 利用している効果音の配布元  
M+フォント([M+フォント](https://mplus-fonts.osdn.jp)) : 利用しているフォントの配布元

---

## ライセンス
MIT License([LICENSE-MIT](https://opensource.org/licenses/MIT))若しくはApache License Version 2.0([LICENSE-APACHE](https://www.apache.org/licenses/LICENSE-2.0))のいずれかとします。
