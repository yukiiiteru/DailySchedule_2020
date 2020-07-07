# Leetcode by Rust

## Problem 1 两数之和

题目链接：[两数之和](https://leetcode-cn.com/problems/two-sum/)

### 题目描述

给定一个整数数组 `nums` 和一个目标值 `target`，请你在该数组中找出和为目标值的那 **两个** 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

**示例**:

> 给定 nums = [2, 7, 11, 15], target = 9
>
> 因为 nums[**0**] + nums[**1**] = 2 + 7 = 9
>
> 所以返回 **[0, 1]**

### 题解

#### Rust

```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let n = nums[i];
            let sub = target - n;
            if let Some(res) = hashmap.get(&sub) {
                return vec![*res, i as i32];
            }
            hashmap.insert(n, i as i32);
        }
        Vec::<i32>::new()
    }
}
```

#### Python

```python
class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """
        d = {}
        for i in range(len(nums)):
            n = nums[i]
            if d.get(target-n) is not None:
                return [d[target-n], i]
            else:
                d[n] = i
        print(d)
```

## Problem 7 整数反转

题目链接：[整数反转](https://leetcode-cn.com/problems/reverse-integer/)

### 题目描述

给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

**示例 1**:

> **输入**: 123
>
> **输出**: 321

**示例 2**:

> **输入**: -123
>
> **输出**: -321

**示例 3**:

> **输入**: 120
>
> **输出**: 21

**注意**:

假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [$−2^{31}$,  $2^{31}-1$]。请根据这个假设，如果反转后整数溢出那么就返回 0。

### 题解

#### Rust

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut res: i32 = 0;
        loop {
            res = match res.checked_mul(10) {
                Some(n) => n + num % 10,
                None    => return 0,
            };
            num /= 10;
            if num == 0 { break; }
        }
        res
    }
}
```

#### Python

```python
class Solution(object):
    def reverse(self, x):
        """
        :type x: int
        :rtype: int
        """
        a = abs(x)
        s = str(a)
        res = int(s[::-1])
        if x < 0:
            res = -res
        if not (-2**31 <= res <= 2**31-1):
            return 0
        return res
```

## Problem 9 回文数

题目链接：[回文数](https://leetcode-cn.com/problems/palindrome-number/)

### 题目描述

判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

**示例 1**:

> **输入**: 121
>
> **输出**: true

**示例 2**:

> **输入**: -121
>
> **输出**: false
>
> **解释**: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。

**示例 3**:

> **输入**: 10
>
> **输出**: false
>
> **解释**: 从右向左读, 为 01 。因此它不是一个回文数。

**进阶**:

你能不将整数转为字符串来解决这个问题吗？

### 题解

#### Rust

```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; }
        let mut num = x;
        let mut res: i32 = 0;
        loop {
            res = res * 10 + num % 10;
            num /= 10;
            if num == 0 { break; }
        }
        res == x
    }
}
```

#### Python

```python
class Solution(object):
    def isPalindrome(self, x):
        """
        :type x: int
        :rtype: bool
        """
        return (str(x) == str(x)[::-1])
```

## Problem 14 最长公共前缀

题目链接：[最长公共前缀](https://leetcode-cn.com/problems/longest-common-prefix/)

### 题目描述

编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 `""`。

**示例 1**:

> **输入**: ["flower","flow","flight"]
>
> **输出**: "fl"

**示例 2**:

> **输入**: ["dog","racecar","car"]
>
> **输出**: ""
>
> **解释**: 输入不存在公共前缀。

**说明**:

所有输入只包含小写字母 `a-z`。

### 题解

#### Rust

```rust
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 { return String::from(""); }
        let mut min_len = std::usize::MAX;
        for s in &strs {
            if s.len() < min_len {
                min_len = s.len();
            }
        }
        for i in 0..min_len {
            let first_ch = &strs[0].as_bytes()[i];
            for j in 1..strs.len() {
                let ch = &strs[j].as_bytes()[i];
                if first_ch != ch {
                    return String::from(&strs[0][0..i]);
                }
            }
        }
        String::from(&strs[0][0..min_len])
    }
}

```

#### Python

```python
class Solution(object):
    def longestCommonPrefix(self, strs):
        """
        :type strs: List[str]
        :rtype: str
        """
        return strs[0][:([len(set(ch))==1 for ch in zip(*strs)] + [False]).index(False)] if strs else ''
        # 注：这段代码我已经忘记了当时是出于什么心态写的
        # 虽然能过题目，但是我已经基本看不懂了
        # 核心是用`zip`构造一个`set`判断其大小
        # 但是这个`False`是干啥我就很懵
```

## Problem 20 有效的括号

题目链接：[有效的括号](https://leetcode-cn.com/problems/valid-parentheses/)

### 题目描述

给定一个只包括 `'('`，`')'`，`'{'`，`'}'`，`'['`，`']'` 的字符串，判断字符串是否有效。

有效字符串需满足：

1. 左括号必须用相同类型的右括号闭合。
2. 左括号必须以正确的顺序闭合。

注意空字符串可被认为是有效字符串。

**示例 1**:

> **输入**: "()"
>
> **输出**: true

**示例 2**:

> **输入**: "()[]{}"
>
> **输出**: true

**示例 3**:

> **输入**: "(]"
>
> **输出**: false

**示例 4**:

> **输入**: "([)]"
>
> **输出**: false

**示例 5**:

> **输入**: "{[]}"
>
> **输出**: true

### 题解

#### Rust

```rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
       let mut v: Vec<char> = Vec::new();
       for ch in s.chars() {
           if ch == '(' || ch == '[' || ch == '{' {
               v.push(ch);
           } else {
               let c = match v.pop() {
                   Some(c)  => c,
                   None     => return false,
               };
               if !(c == '(' && ch == ')' ||
                  c == '[' && ch == ']' ||
                  c == '{' && ch == '}') {
                    return false;
               }
           }
       }
       v.len() == 0
    }
}
```

#### Python

```python
class Solution(object):
    def isValid(self, s):
        """
        :type s: str
        :rtype: bool
        """
        stack = []
        bracket = {')': '(', ']': '[', '}': '{'}
        for ch in s:
            if ch in bracket:
                if not stack:
                    return False
                if bracket[ch] != stack.pop():
                    return False
            else:
                stack.append(ch)
        return not stack
```
