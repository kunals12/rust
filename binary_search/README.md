# 📘 Binary Search — Notes & Explanation

This document contains simple, clear, and coding-friendly notes on **Binary Search** for your personal understanding and future reference.

---

## ✅ What is Binary Search?

Binary Search is a searching algorithm used to find an element in a **sorted array**.

👉 **Important condition:**
Binary search works **only if the array is sorted** (ascending order).

If the array is not sorted → binary search **cannot be used**.

---

## 🎯 Core Idea (in one line)

> **Check the middle → throw away half → repeat.**

Every step reduces the search space by half.

---

## 🔍 How Binary Search Works (Step-by-Step)

Given a sorted array:

```
[2, 5, 8, 12, 16, 23, 38]
```

and target = `12`

### Step 1:

Set two pointers:

```
low = 0
high = n - 1   // last index
```

Compute:

```
mid = (low + high) / 2
```

Check `arr[mid]`:

* If `arr[mid] == target` → FOUND ✅
* If `arr[mid] < target` → search **right half**

```
low = mid + 1
```

* If `arr[mid] > target` → search **left half**

```
high = mid - 1
```

Repeat until `low > high` (not found) or target is found.

---

## 🧠 Why Binary Search is Fast

If array size = `n`

| Algorithm     | Worst-case time |
| ------------- | --------------- |
| Linear Search | **O(n)**        |
| Binary Search | **O(log n)** ✅  |

This means:

* 1,000,000 elements → Linear may take 1,000,000 steps
* Binary search takes only about **20 steps**

That’s why it is widely used in systems, databases, and APIs.

---

## 🧾 Standard Pseudocode (for your notes)

```
low = 0
high = n - 1

while low <= high:
    mid = (low + high) / 2

    if arr[mid] == target:
        return mid      // found

    else if arr[mid] < target:
        low = mid + 1   // search right half

    else:
        high = mid - 1  // search left half

return -1   // not found
```

---

## 🧩 Key Properties to Remember

1. Works only on **sorted arrays**
2. Uses **three pointers**: `low`, `high`, `mid`
3. Eliminates **half of the search space** every step
4. Time complexity: **O(log n)**
5. Space complexity: **O(1)** (iterative version)

---

## 📌 Connection to Your Code (for context)

Your current example uses **Linear Search** and counts unsuccessful attempts.

If you later implement binary search, the idea would change from:

> “Check every element one by one”

to:

> “Jump to the middle, cut the array, and continue.”

If you want, I can:

* add a **Rust binary search function**, or
* modify your program to **count attempts in binary search** as well.
