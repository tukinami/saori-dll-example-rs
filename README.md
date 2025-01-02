# saori-dll-example-rs

[GitHub repository](https://github.com/tukinami/saori-dll-example-rs)

## これは何?

デスクトップマスコット、「伺か」用DLLの一種、「SAORI」の例です。

拙作「[ukagaka-dll-macro](https://github.com/tukinami/ukagaka-dll-macro)」、
「[saori-interface-rs](https://github.com/tukinami/saori-interface-rs)」の使用例でもあります。

SAORIの機能としては、RGBの色の割合(0.0 ~ 1.0)から`sRGB`としてみたときの光度を返します。

## 雛形としてのプロジェクト

雛形として、プロジェクトをコピーして、改造することを想定しています。

ライブラリ以外のソースコードを`CC0 1.0 Universal`でライセンスしているので、ご自由に使用していただけます。
公開・配布するときは、付属の`LICENSES.html`を同梱して下さい。

## SAORIとしての使い方

SAORI自体の使い方は、使用するSHIORIなどによって異なりますので、ご自身でお調べ下さい。

ここではこのSAORIの使い方について説明いたします。

以下の引数を指定して使用します。

+ Argument0: Rの割合(0.0 ~ 1.0)
+ Argument1: Gの割合(0.0 ~ 1.0)
+ Argument2: Bの割合(0.0 ~ 1.0)

### 成功したとき

Resultに計算結果が半角数値で入ります。

## 使用ライブラリ

いずれも拙作。

+ [ukagaka-dll-macro](https://github.com/tukinami/ukagaka-dll-macro)
+ [saori-interface-rs](https://github.com/tukinami/saori-interface-rs)

## ライセンス

上記ライブラリとその関連以外の、このプロジェクトのソースコードに対して、以下のライセンスを適用します。

<p xmlns:cc="http://creativecommons.org/ns#" xmlns:dct="http://purl.org/dc/terms/"><a property="dct:title" rel="cc:attributionURL" href="https://github.com/tukinami/saori-dll-example-rs">saori-dll-example-rs</a> by <a rel="cc:attributionURL dct:creator" property="cc:attributionName" href="https://github.com/tukinami">月波 清火 (tukinami seika)</a> is marked with <a href="https://creativecommons.org/publicdomain/zero/1.0/?ref=chooser-v1" target="_blank" rel="license noopener noreferrer" style="display:inline-block;">CC0 1.0 Universal<img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1" alt=""><img style="height:22px!important;margin-left:3px;vertical-align:text-bottom;" src="https://mirrors.creativecommons.org/presskit/icons/zero.svg?ref=chooser-v1" alt=""></a></p>

あなたがこのプロジェクトに直接貢献した場合、同ライセンスの下、あなたの貢献したものがこのプロジェクトの一部としてパブリックドメインになることに同意したものとみなされます。

## 作成者

月波 清火 (tukinami seika)

[GitHub](https://github.com/tukinami)
