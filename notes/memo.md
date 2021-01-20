### warning: second app.draw() will reset before one

`app.draw()`で Drawing struct を参照したあと、
出力するまでに別の場所でもう一回実行するとリセットされる
