---
datetime=2023-07-25 10:32:00Z

url_name="promise-study"

title="Promise 的学习"

tags= ["ES6"]

category= "FE"
---

# 📝Promise

**promise**  :对象用于表示一个异步操作的最终完成 (或失败)及其结果值。

一个 Promise 对象代表一个在这个 promise 被创建出来时不一定已知的值。它让您能够把异步操作最终的成功返回值或者失败原因和相应的处理程序关联起来。这样使得异步方法可以像同步方法那样返回值：异步方法并不会立即返回最终的值，而是会返回一个 promise，以便在未来某个时候把值交给使用者。

一个 Promise 必然处于以下几种状态之一：

- 待定（pending）: 初始状态，既没有被兑现，也没有被拒绝。
- 已兑现（fulfilled）: 意味着操作成功完成。
- 已拒绝（rejected）: 意味着操作失败。

待定状态的 Promise 对象要么会通过一个值被兑现（fulfilled），要么会通过一个原因（错误）被拒绝（rejected）。当这些情况之一发生时，我们用 promise 的 then 方法排列起来的相关处理程序就会被调用。如果 promise 在一个相应的处理程序被绑定时就已经被兑现或被拒绝了，那么这个处理程序就会被调用，因此在完成异步操作和绑定处理方法之间不会存在竞争状态**。**

## promise.all(iterable):

完成（fulfillment):

当传入的参数是一个空的可迭代对象时，则返回一个已完成（resolved）的 promise

如果所有传入的 promise 都变为完成状态，或者传入的可迭代对象内没有 promise，Promise.all 返回的 promise 异步地变为完成。

在任何情况下，Promise.all 返回的 promise 的完成状态的结果都是一个数组，它包含所有的传入迭代参数对象的值（也包括非 promise 值）。

失败/拒绝（Rejection）：

如果传入的 promise 中有一个失败（rejected），Promise.all 异步地将失败的那个结果给失败状态的回调函数，而不管其它 promise 是否完成。

**代码实现**

`https://github.com/Liboq/xiaobais-web/blob/main/%E5%AD%97%E8%8A%82%E9%A3%9E%E4%B9%A6/code/06-promise.all.js`

## promise.reject

**MDN:    Promise.reject()**方法返回一个带有拒绝原因的 Promise 对象。

`https://github.com/Liboq/xiaobais-web/blob/main/%E5%AD%97%E8%8A%82%E9%A3%9E%E4%B9%A6/code/08-Promise.reject.js`

## promise.resolved

MDN:Promise.resolve(value)方法返回一个以给定值解析后的 Promise 对象。如果这个值是一个 promise ，那么将返回这个 promise ；如果这个值是 thenable（即带有"then" 方法），返回的 promise 会“跟随”这个 thenable 的对象，采用它的最终状态；否则返回的 promise 将以此值完成。此函数将类 promise 对象的多层嵌套展平。

`https://github.com/Liboq/xiaobais-web/blob/main/%E5%AD%97%E8%8A%82%E9%A3%9E%E4%B9%A6/code/07-promise.reslove.js`

## promise.race

字面意思就是赛跑，以变化最快的 promise 为准，它若是失败则返回 reject，他若是成功，则成功

MDN:如果传的迭代是空的，则返回的 promise 将永远等待。

如果迭代包含一个或多个非承诺值和/或已解决/拒绝的承诺，则 Promise.race 将解析为迭代中找到的第一个值。

`https://github.com/Liboq/xiaobais-web/blob/main/%E5%AD%97%E8%8A%82%E9%A3%9E%E4%B9%A6/code/10-promise.race.js`

## promise.any

他和 promise.all 时相反的，他只要有一个成功，则显示成功那项

MDN:这个方法用于返回第一个成功的 promise 。只要有一个 promise 成功此方法就会终止，它不会等待其他的 promise 全部完成。

不像 Promise.all() 会返回一组完成值那样（resolved values），我们只能得到一个成功值（假设至少有一个 promise 完成）。当我们只需要一个 promise 成功，而不关心是哪一个成功时此方法很有用的。

同时, 也不像 Promise.race() 总是返回第一个结果值（resolved/reject）那样，这个方法返回的是第一个 成功的 值。这个方法将会忽略掉所有被拒绝的 promise，直到第一个 promise 成功。

`https://github.com/Liboq/xiaobais-web/blob/main/%E5%AD%97%E8%8A%82%E9%A3%9E%E4%B9%A6/code/11-promise.any.js`

## promise.allSettled

该 Promise.allSettled()方法返回一个在所有给定的 promise 都已经 fulfilled 或 rejected 后的 promise，并带有一个对象数组，每个对象表示对应的 promise 结果。

当您有多个彼此不依赖的异步任务成功完成时，或者您总是想知道每个 promise 的结果时，通常使用它。

`https://github.com/Liboq/xiaobais-web/blob/main/%E5%AD%97%E8%8A%82%E9%A3%9E%E4%B9%A6/code/12.promise.allSettled.js`

<aside>
💡 欢迎您在底部评论区留言，一起交流~

</aside>
