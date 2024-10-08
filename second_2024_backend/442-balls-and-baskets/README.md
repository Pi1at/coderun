# 442. Шары и корзины

Перед вами $n$ корзин с шарами, в $i-$й корзине лежит $a_i$ шаров. Вам поступают два вида запросов:

- `0 l r` — положить в корзины с номерами от $l$ до $r$ по 1 дополнительному шару;
- `1 l r` — посчитать, сколько существует способов выбрать ровно по 1 шару из каждой корзины с номерами от $l$ до $r$. Все шары считаются различными, а два способа считаются различными, если существует корзина, из которой были вытащены различные шары.

Дополнительно гарантируется, что в каждую корзину будет добавлено не более 400 дополнительных шаров.

Поскольку количество способов выбрать шары из корзин может быть очень большим, требуется вывести остаток от деления этого числа на $10^9 + 7$.

## Формат ввода

Первая строка содержит число $n$ $(1 \le n \le 100\,000)$ — количество корзин.

Вторая строка содержит последовательность из целых чисел $a_i$ $(1 \le a_i \le 10^9)$, разделённых пробелом, — изначальное количество шаров в каждой корзине.

Третья строка содержит число $q$ $(1 \le q \le 100\,000)$ — количество запросов.

Следующие $q$ строк содержат запросы в формате, описанном выше.

## Формат вывода

Для каждого запроса вида `1 l r` выведите в новой строке остаток от деления количества способов выбрать по одному шару из каждой корзины соответствующего диапазона на $10^9 + 7$.

## Примечание

В приведённом примере изначально есть 6 способов выбрать по шару из каждой корзины. Второй запрос добавляет по шару в первую и вторую корзины, а третий запрос относится к первой и второй корзинам, после прошлого запроса содержащим 2 и 3 шара соответственно.

## Ограничения

Ограничение времени

3 с

Ограничение памяти

256 МБ

## Пример 1

| Ввод  | Вывод |
|-------|-------|
| 3     | 6     |
| 1 2 3 | 6     |
| 3     |       |
| 1 1 3 |       |
| 0 1 2 |       |
| 1 1 2 |       |
