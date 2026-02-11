# 📘 Bubble Sort — Notes & Explanation

This document explains **Bubble Sort** in a simple, intuitive, and coding-friendly way for your personal notes.

---

## ✅ What is Bubble Sort?

Bubble Sort is a **comparison-based sorting algorithm** that repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order.

The larger elements “bubble up” to the right — hence the name **Bubble Sort** 🫧

---

## 🎯 Core Idea (one line)

> **Compare neighbours → swap if out of order → repeat until sorted.**

---

## 🔍 How Bubble Sort Works (Intuition)

Take this array:

```
[4, 5, 8, 9, 1, 6]
```

### Pass 1 (i = 0)

Compare adjacent pairs:

```
4 vs 5 → OK
5 vs 8 → OK
8 vs 9 → OK
9 vs 1 → SWAP  → [4,5,8,1,9,6]
1 vs 6 → OK
```

Largest element (9) has moved toward the end.

---

### Pass 2 (i = 1)

Now last element is fixed, so we check one less position:

```
4 vs 5 → OK
5 vs 8 → OK
8 vs 1 → SWAP → [4,5,1,8,9,6]
1 vs 8 → OK
8 vs 9 → OK
9 vs 6 → SWAP → [4,5,1,8,6,9]
```

Now 9 is correctly placed.

---

This continues until the array is fully sorted.

---

## 🔁 Why Two Loops?

### Outer loop (`i`)

Controls how many passes we make over the array.

Each pass fixes **one largest element at the end**.

### Inner loop (`j`)

Actually compares and swaps adjacent elements.

---

## 🔄 Swap Logic (Most Important Part)

To swap two values `a` and `b`, we use a temporary variable:

```
temp = a
a = b
b = temp
```

This ensures we don’t lose the original value of `a`.

---

## ⏱ Time Complexity

| Case    | Time                                 |
| ------- | ------------------------------------ |
| Best    | O(n)  (already sorted, breaks early) |
| Worst   | O(n²)                                |
| Average | O(n²)                                |

Bubble sort is **simple but not efficient for large data**.

---

## 🧠 Key Things to Remember

1. Works by comparing **adjacent elements**
2. Uses **swapping**
3. Largest elements move to the **right end**
4. Can stop early if no swaps happen
5. Good for learning, not for big systems

---
