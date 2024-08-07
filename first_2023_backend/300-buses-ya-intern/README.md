# 300. Первый герой

Самолёт героев телесериала «Первый герой» потерпел крушение, и они оказались на необитаемом острове. Героям повезло: уцелела карта полётов, которая была у пилотов самолета. По этой карте они смогли определить, какие рейсы пролетают на достаточно близком расстоянии, чтобы их сигнальные костры были замечены. Среди героев оказался профессор Арифметик, которого заинтересовал вопрос: какое математическое ожидание времени ожидания первого пролетающего мимо самолета.

В рамках этой задачи он использовал следующие ограничения:

- все авиарейсы независимы;
- интервалы вылета одного маршрута постоянны;
- все маршруты начинают движения в произвольное время, до того как они потерпели крушение (к сожалению, ни одних целых часов не осталось, и даже координат своего местоположения профессор не знает).

Другими словами, если $k$-ый авиарейс летает с интервалом $t_k$ часов и первый раз после крушения он пролетит мимо через $s_k$ часов, то $s_k$ — случайная величина, равномерно распределенная на полуинтервале $[0; \, t_k)$.

## Формат ввода

Первая строка входных данных содержит единственное число $N$ — количество маршрутов $(1 \le N \le 5)$. Вторая строка содержит $N$ целых положительных чисел $t_k$ $(1 \le t_k \le 50)$.

## Формат вывода

В единственной строке выведите математическое ожидание времени ожидания первого самолета в виде несократимой дроби (в часах).