# 257. RLE-сжатие

- средняя
- binary search
- strings
- first\_2023\_backend

RLE-сжатие — один из самых простых методов сжатия строки, основанный на сокращении подстрок, состоящих из одинаковых символов. Сжатие осуществляется следующим образом:

- Строка разбивается на минимальное количество подстрок, состоящих из одинаковых символов. Например, `abbcaaa` превращается в строки `a`, `bb`, `c`, `aaa`.
- Каждая из полученных строк превращается в строку, состоящую из числа и буквы. Числом является количество повторений символа в этой строке, буква берётся из первого символа обрабатываемой строки. Число не добавляется, если количество символов в строке равно единице. Из предыдущего массива строк мы получаем `a`, `2b`, `c`, `3a`.
- Затем полученные строки конкатенируются в исходном порядке. В рассмотренном примере в итоге получим `a2bc3a`.

Вам дана строка $s$, уже сжатая в $RLE$-формате. Назовём строку, из которой была получена $s$, строкой $t$. Вам даны $q$ запросов, каждый из них представлен целыми числами $l$ и $r$. В каждом запросе вам необходимо найти длину `сжатой` подстроки $t[l \ldots r]$.

## Формат ввода

В первой строке входного файла записана строка $s$, состоящая из строчных букв латинского алфавита и цифр $(1 \le |s| \le 1\,000\,000)$. Гарантируется, что существует такая непустая строка $t$, из которой $RLE$-сжатием получается строка $s$. Также гарантируется, что в строке $t$ не было больше $1\,000\,000\,000$ одинаковых подряд идущих символов.

В следующей строке дано количество запросов $q$ $(1 \le q \le 100\,000)$. Каждая из следующих $q$ строк содержит два числа $l_i$ и $r_i$ $(1 \le l_i \le r_i \le |t|)$ — параметры запросов.

## Формат вывода

Выведите $q$ чисел, каждое в отдельной строке — ответы на запросы в том порядке, в котором запросы были заданы во входных данных.

### Пример 1

Ввод   | Вывод
-------|------
a2bc3a | 6
5      | 2
1 7    | 2
5 7    | 3
1 2    | 1
3 5    |
4 4    |

### Пример 2

Ввод             | Вывод
-----------------|------
x1000000000yz    | 11
3                | 12
2 1000000001     | 9
2 1000000002     |
5938493 15938493 |
