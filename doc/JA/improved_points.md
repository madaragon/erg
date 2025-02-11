# Pythonから改良された点

## 静的解析を行う(静的型チェック、変数・プロパティチェック)

静的型チェックの恩恵は今更強調するまでもないほどですが、変数・プロパティの存在チェックもかなり効いてくる部分です。

## 厳密にスコープを扱う

Pythonでは文がスコープを持ちません。
そのため、`for`や`if`の中で定義した変数は外に影響を与えてしまいます。気軽に変数を名付けることができません。

```python
for i in range(10):
    x = 1
    print(i + x)
print(x) # 1
```

Ergでは全てのブロックがスコープを持ち、完全に隔離されています。

## 可変オブジェクトと不変オブジェクトの区別が明確

Pythonは可変オブジェクトと不変オブジェクト、ヒープオブジェクトと値オブジェクトの区別が明確ではないため、タプルは不変だがリストは可変...などと言った知識を頭に入れておく必要があります。
また、自作クラスを不変にしたいとき、面倒な手順をふまなくてはなりません。

```python
# このコードが過去のPythonでは有効だったと信じられますか?
i = 256
assert i is 256
i = 257
assert i is not 257
```

## トレイトを持つ

Javaのインターフェースと同じように、契約に基づくプログラミングを行うことができます。

Pythonにも抽象基底クラスがありますが、この手の構造体は静的型付けと組み合わせることで最大の効果を発揮します。

## 依存関係を静的に解決する

長時間実行の末にモジュールが足りずエラーなどといったゲンナリする体験を未然に防ぎます。

## 組み込みのパッケージマネージャー

規格化されたディレクトリ構造とビルドファイルを用いて再現性のあるビルドが可能です。
ロックファイルの生成やバージョン管理はもちろん行われます。
anacondaやらpyenvやらpoetryやらをプロジェクトごとに取捨選択したり組み合わせたりする必要はありません。
