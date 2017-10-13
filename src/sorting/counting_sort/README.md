# Counting sort

[Counting sort][wiki-counting-sort] 是一個特殊的整數排序法，被視為 Bucket sort 的特例。原理是在已知整數範圍內，計算每個鍵值出現次數，並用額外的陣列保存（Count array）。最後將 Count array 的元素值作為排序資料的新 index。

Counting sort 基本特性如下：

- **非原地演算**：額外花費較大量，非固定的空間來排序。
- **穩定排序**：相同鍵值的元素，排序後相對位置不改變。
- **整數排序**：以整數作為排序的鍵值。
- **分配式排序**：不透過兩兩比較，而是分析鍵值分佈來排序。特定情況下可達線性執行時間。
- **線型執行時間**：當輸入資料量 **n** 與已知範圍上下界之差值相近，執行時間接近線型（**O(n)**）
- **預期分佈**：預期輸入資料是落在已知範圍內的整數（例如 0 到 k）。
- **適用範圍**：僅適用於小範圍整數（額外空間需求大）。

## Algorithm

1. **Count occurrence**：計算每個 key 的出現次數。
2. **Prefix sum as start index**：計算前綴和（Prefix sum），並作為該元素的 start index。
3. **Copy output**：利用步驟二的前綴和，遍歷輸入資料，取得元素排序後的索引。

## Explanation

這裡有資料需要透過正整數的 key 來排序。key 的範圍在 0 - 9 之間，格式為 `(key, value)`。

```
Input: (1, A) (5, B) (8, C) (2, D) (2, E) (9, F)
```

**1. Count occurrence**：首先，先計算每個 key 的出現頻率，儲存在額外的 count array 中。

```
Key  : 0 1 2 3 4 5 6 7 8 9
Count: 0 1 2 0 0 1 0 0 1 1
```

**2. Prefix sum as start index**：再計算 prefix sum，也就是將當前 index 前累計的 key 數量加總。例如 **key 5** 的 prefix sum **1 + 2 = 3**。

這裡的 prefix sum 等同於每筆資料排序後的位置（index）。例如排序後，**8** 位於陣列第四位。

```
Key       : 0 1 2 3 4 5 6 7 8 9
Prefix Sum: 0 0 1 3 3 3 4 4 4 5
```

**3. Copy output**：透過 key 與 prefix sum 的映射關係，找到原始資料對應的位置。

實作上，每筆資料找到對應的 start index（prefix sum） 後，要將**該 index 之值 +1**，使得重複的元素可取得正確的 index offset（對唯一的 key 沒有影響）。

```
(1, A)
--> prefix sum 為 0，寫入 array[0]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) |        |        |        |        |        |
+--------+--------+--------+--------+--------+--------+

(5, B)
--> prefix sum 為 3，寫入 array[3]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) |        |        | (5, B) |        |        |
+--------+--------+--------+--------+--------+--------+

(8, C)
--> prefix sum 為 4，寫入 array[4]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) |        |        | (5, B) | (8, C) |        |
+--------+--------+--------+--------+--------+--------+

(2, D)
--> prefix sum 為 2，寫入 array[4]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) | (2, D) |        | (5, B) | (8, C) |        |
+--------+--------+--------+--------+--------+--------+

(2, E)
--> prefix sum 為 3（前一步驟 + 1），寫入 array[3]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) | (2, D) | (2, E) | (5, B) | (8, C) |        |
+--------+--------+--------+--------+--------+--------+

(9, F)
--> prefix sum 為 5，寫入 array[5]，並將 prefix sum + 1

+--------+--------+--------+--------+--------+--------+
| (1, A) | (2, D) | (2, E) | (5, B) | (8, C) | (9, F) |
+--------+--------+--------+--------+--------+--------+
```

這樣就完成排序了。此外，觀察 **(2, D)** 與 **(2, E)** 排序前後的位置，會發現 counting sort 是個實實在在的穩定排序，很棒。

## Performance

|              | Complexity           |
| :----------- | :------------------- |
| Worst        | $O(n + k)$           |
| Best         | $\Omega(n + k)$      |
| Average      | $\Theta(n + k)$      |
| Worst space  | $O(n + k)$ auxiliary |

> k 為資料已知範圍上下界之差。

### Time complexity

Counting sort 沒有用到任何遞迴，可以直觀地分析複雜度。在步驟一，建立 count array 與步驟三輸出排序結果，都需要遍歷 $n$ 個輸入的資料，因此複雜度為 $O(n)$；步驟二計算 prefix sum，，以及 count array 自身的初始化則需執行 $k + 1$ 次（給定的資料範圍），這部分的複雜度為 $O(k)$。由於 $n$ 與 $k$ 的權重會因輸入資料及實作的不同而有所改變，我們無法捨棄任何一個因子，可得知 counting sort 的複雜度為 $O(n + k)$。

### Space complexity

Counting sort 並非 in-place sort，排序後的結果會另外輸出為新的記憶體空間，因此 $O(n)$ 的額外（auxiliary）空間複雜度絕對免不了。再加上需要長度為 $k$ 的 count array 保存每個 key 的出現次數，因此需再加上 $O(k)$。除了原始的輸入 array，總共需額外花費 $O(n + k)$ 的空間複雜度。

> 如果欲排序資料就是整數自身，可以將「計算前綴和」與「複製輸出」兩步驟最佳化，直接覆寫原始陣列，空間複雜度也會下降至 $O(k)$ 了。

## Reference

- [Wiki: Counting sort][wiki-counting-sort]
- [Growing with the web: Counting sort][growingwiththeweb]

[wiki-counting-sort]: https://en.wikipedia.org/wiki/Counting_sort
[growingwiththeweb]: http://www.growingwiththeweb.com/2014/05/counting-sort.html
