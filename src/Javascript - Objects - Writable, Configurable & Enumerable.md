
---

- **writable**: 我可以修改他们的值，我可以更新一个新的值给属性。（默认true）
- **enumerable**: 我可以用 for in 访问到他们，且enumerable是使用 [Object.keys](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/keys) 的返回。
- **configurable**: 我可以修改这个属性的所有，包括 non-enumerable, non-writable 甚至 non-cofigurable，使用 delete 关键字操作只有在 configurable 为 true 才能操作。

---
- **writable**: I can modify their values, I can update a property just assigning a new value to it.Defaults to <span style="background-color: #9b9a94;color:black">true</span>.
- **enumerable**: I can access to all of them using a for..in loop. Also, enumerable property keys of an object are returned using [Object.keys](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/keys) method.** **Defaults to <span style="background-color: #9b9a94;color:black">true</span>.
- **configurable**: I can modify the behavior of the property, so I can make them non-enumerable, non-writable or even non-cofigurable if I feel like doing so. Configurable properties are the only ones that can be removed using the delete operator. Defaults to <span style="background-color: #9b9a94;color:black">true</span>.
