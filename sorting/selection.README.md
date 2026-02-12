# 📘 Selection Sort — Notes & Explanation

This document explains **Selection Sort** in a simple, intuitive, and coding-friendly way for your personal notes.

---

## ✅ What is Selection Sort?

Selection Sort is a comparison-based sorting algorithm that:

* Selects the **smallest element** from the unsorted part
* Places it at the **current position**
* Shrinks the unsorted part and repeats

Unlike Bubble Sort, it **does not repeatedly swap adjacent elements** — it selects first, then swaps once per pass.

---

## 🎯 Core Idea (one line)

> **Find the smallest → place it at the front → repeat.**

---

## 🔍 How Selection Sort Works (Intuition)

Take this array:

```
[4, 2, 7, 5, 8]
```

### Pass 1 (i = 0)

Unsorted part: `[4, 2, 7, 5, 8]`
Smallest = **2** → swap with index 0

Result:

```
[2, 4, 7, 5, 8]
```

---

### Pass 2 (i = 1)

Unsorted part: `[4, 7, 5, 8]`
Smallest = **4** → already in correct place → no effective change

Result:

```
[2, 4, 7, 5, 8]
```

---

### Pass 3 (i = 2)

Unsorted part: `[7, 5, 8]`
Smallest = **5** → swap with index 2

Result:

```
[2, 4, 5, 7, 8]
```

---

### Pass 4 (i = 3)

Unsorted part: `[7, 8]`
Smallest = **7** → already correct

Final sorted array:

```
[2, 4, 5, 7, 8]
```

---

## 🔁 Why Two Loops?

### Outer loop (`i`)

Marks the boundary between:

* **Sorted part (left side)**
* **Unsorted part (right side)**

Each pass fixes **one position permanently**.

### Inner loop (`j`)

Searches for the **minimum element** in the remaining unsorted part.

---

## 🔄 Swap Logic (same as Bubble Sort)

To swap two positions `a` and `b`:

```
temp = arr[a]
arr[a] = arr[b]
arr[b] = temp
```

This prevents losing values.

---

## ⏱ Time Complexity

| Case    | Time  |
| ------- | ----- |
| Best    | O(n²) |
| Worst   | O(n²) |
| Average | O(n²) |

👉 Unlike Bubble Sort, Selection Sort **does NOT become faster for sorted arrays** — it still checks all elements.

---

## 🧠 Key Differences vs Bubble Sort

| Feature    | Bubble Sort               | Selection Sort            |
| ---------- | ------------------------- | ------------------------- |
| Swaps      | Many swaps                | Only one swap per pass    |
| Finds      | Wrong neighbours          | Minimum element           |
| Early stop | Yes                       | ❌ No                      |
| Idea       | “Push big elements right” | “Pick smallest each time” |

---

## 🧾 Standard Pseudocode (for your notes)

```
for i = 0 to n-2:
    min = i

    for j = i+1 to n-1:
        if arr[j] < arr[min]:
            min = j

    swap(arr[i], arr[min])
```
