# Interval Type

`Range`オブジェクトの最も基本的な使い方は、イテレータとしての使用です。

```erg
for! 0..9, i =>
    print! i
```

Pythonと違い、末尾の数字は含まれることに注意してください。

しかし、`Range`オブジェクトの使い道はこれだけではありません。型としても使うことが出来ます。このような型を区間型(Interval type)と呼びます。

```erg
i: 0..10 = 2
```

`Nat`型は`0..<Inf`、`Int`, `Ratio`型は`-Inf<..<Inf`と等価な型です。
`0..<Inf`は`0.._`と書くことも出来ます。`_`は、`Int`型の任意のインスタンスを意味します。

イテレータとしても使えるため、`10..0`などのように逆順で指定することも出来ますが、
`<..`, `..<`, `<..<`の向きは逆転できません。

```erg
```

範囲演算子(range operator)は、`Ord`な不変型であるならば数値以外の型にも使用できます。

```erg
Alphabet = "A".."z"
```
