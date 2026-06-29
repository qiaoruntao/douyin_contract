# douyin_contract

中文 | [English](README_EN.md)

本项目的核心是一条流水线：

```
抖音前端 JS  --(tools/extract_proto_from_webpack_chunk.py)-->  .proto + mapping.json
```

下游使用者（例如 `douyin_live_fetcher`）依赖本仓库生成的 proto（通过本仓库同时构建的
Rust crate）来获取消息结构，从而专注于抓取/解码逻辑，而不必自己逆向协议和维护 schema。

## 为什么要做这件事

抖音直播间的 wss 协议（弹幕、礼物、粉丝团、排行榜、红包……）没有任何官方文档。但 schema
其实完整地存在于 live.douyin.com 发给每个浏览器的前端 JS 里 —— 以 protobuf.js 编译出的
“静态模块”形式存在（每个消息类型的 `decode()` 函数里手写编码了自己的
字段号 → 字段名 → 类型 对照表）。这足以机械化地还原出一份真实的 `.proto`，而且因为它直接来自
生产环境的 bundle，只要重新跑一遍脚本抓一份新的 chunk，就能跟着抖音协议的演进保持准确。

## 流水线

`tools/extract_proto_from_webpack_chunk.py` 接收一个输入（JS chunk），产出两个输出
（一份 `.proto` 和一份 `mapping.json`）：

1. **定位命名空间/类型树。** 用一个对字符串敏感的括号匹配器遍历源码（避免字符串/模板字面量
   里的花括号打乱括号深度计数），还原出 `Webcast.Im.*` / `Webcast.Data.*` 的包含关系树 ——
   `Im` 下面是所有真正在 wss 上发送的消息类型；`Data` 下面是被 `Im` 类型引用的公共结构体
   （`User`、`Image` 等）。
2. **提取每个类型自己的字段。** 每个消息的 `decode()` 函数会内联构建一张字段表，例如
   `a={1:["nickname",e.string,0],2:["avatar_thumb",r.webcast.data.Image.decode,1],...}`
   （字段号 → [JS 字段名，解码函数引用，repeated 标志位]）。这张表是用括号深度感知的扫描定位
   出来的，不是单一正则 —— 假设只有一层括号嵌套的正则会在大表上悄无声息地截断（例如 `User`
   真实有约 70 个字段，正则的嵌套假设覆盖不到这么多条目）。提取前会先把嵌套子类型自己的字段表
   遮蔽掉，否则一个带有较大嵌套子类型的类型，最终可能被错误地输出成子类型的字段而非它自己的字段
   （这是这个项目第一次真实测试时发现并修复的问题 —— 如果好奇可以在 git 历史里看
   `User` vs `User.PayGrade` 那次提交）。
3. **单独提取 `map<K,V>` 字段。** protobuf.js 对 map 字段会生成专门的内联解码代码，
   *完全在*通用字段表之外（一个 map entry 在协议上被编码成一个两字段的内嵌消息：字段 1 是
   key，字段 2 是 value，由一段手写循环解码）—— 所以这类字段对第 2 步来说是完全不可见的，
   不是类型推断错了，是压根没被发现。这里用单独的正则匹配那段内联循环的代码形状。同一种模式
   实际上有两种文本写法：一个类型上只有一个 map 字段时是 `if(N===_){...}`；有两个或更多时会
   变成 `switch(_){case N:...;break;case M:...;break;}` —— 两种写法都已支持
   （`PublicAreaCommon` 和 `MemberMessage` 各有多个 map 字段，用的就是 switch 写法，
   脚本最初的版本完全没认出这种形态）。
4. **推断 proto 类型。** 标量类型从解码函数引用推断（`e.string`→`string`，
   `e.int64String`/`e.uint64String`→`int64`/`uint64` —— protobuf.js 在 JS 端把 64 位数值
   渲染成字符串，因为 JS number 精度放不下，但协议上的 wire format 就是普通的
   int64/uint64，不受影响）。消息类型字段则从解码函数引用的点号路径解析
   （`r.webcast.data.Image.decode` → `Webcast.Data.Image`）。
5. **生成 `mapping.json`。** 即 wire 上 `Message.method` 字符串 → 完整 proto 类型名的映射，
   例如 `"WebcastChatMessage": "Webcast.Im.ChatMessage"`。绝大多数 `Webcast.Im` 下的类型都
   遵循 `method == "Webcast" + 类型名` 这个规律；脚本里一份很小的硬编码覆盖表
   （`KNOWN_METHOD_OVERRIDES`，共 5 条）覆盖了已知的例外 —— 例如 `LinkMicMethod` 根本没有
   "Webcast" 前缀，`WebcastRoomNotifyMessage` 对应的类型是 `NotifyMessage` 而不是
   `RoomNotifyMessage`。这些没法从 JS chunk 本身推导出来（它从不在任何地方嵌入字面的 wire
   method 字符串，只有类型名）—— 这份覆盖表是从 `Remember-the-past/douyin_proto` 的
   方法映射文档里提取一次后硬编码进来的。

### 用法

```
python3 tools/extract_proto_from_webpack_chunk.py <live-schema-im.js 的路径> <输出.proto> [mapping.json]
```

如何找到输入文件：这个 chunk 的 CDN 域名和 hash 会随时间变化（目前见过
`lf-webcast-platform.bytetos.com` 和 `lf-douyin-pc-web.douyinstatic.com` 两个域名，hash
随每次前端发版变化）—— 打开 live.douyin.com，在加载的网络请求里找到
`live-schema-im.<hash>.js` 这个 chunk，直接下载这个 URL。不要把某个 URL 硬编码下来，永远
重新去现场找当前的。

### 已知限制

- **map 的 key/value 类型是从运行时的解码调用推断出来的，不是从声明类型推断的** —— 目前
  观察到的所有情况都覆盖到了（key 全是 int32，value 是标量或消息类型），但如果出现不常见的
  key 类型，需要对 `MAP_FIELD_RE`/`find_map_fields` 做一点小扩展。
- **`mapping.json` 的覆盖表是一份固定表**，不是从 JS 推导出来的。如果抖音引入了一个不遵循
  `"Webcast" + 类型名` 规律的新 method 名，需要手动补充进去（症状是某个
  `Message.method` 字符串在 `mapping.json` 里找不到对应条目）。
- 产出必然只是喂进去的那份 JS chunk 的一个快照。需要定期用新的 chunk 重新跑一遍 ——
  脚本本身没有"当前最新"这个概念，只认输入文件。

## 目录结构

- `tools/extract_proto_from_webpack_chunk.py` —— 生成器本体（见上文）。
- `examples/verify_mongo_roundtrip.rs` —— 校验工具（见上文）。它专用的依赖
  （`prost-reflect`、`mongodb`、`tokio` 等）都放在 `Cargo.toml` 的
  `[dev-dependencies]` 里，不会被本 crate 的使用者一起拉进去。
- `assets/douyin.proto` —— 通过 `prost-build` + `protox`（见 `build.rs`）在构建期编译成
  本 crate 的 Rust 类型。分两部分：一小段手写的信封（envelope）类型
  （`PushFrame`、`HeadersList`、`Response`、`Message` —— 最外层的 wss 帧结构，
  *不在* `live-schema-im.js` 的覆盖范围内，那份 JS 只描述 `Message.payload` 里面的内容），
  后面跟着完整生成的 `Webcast.Im.*` / `Webcast.Data.*` schema。
- `mapping.json`（仓库根目录）—— 生成器的产物，和 `assets/douyin.proto` 配套使用。

之前那份用了多年的、手工整理的 proto2 schema（rust-protobuf 生成，`Im` 前缀的扁平消息名）
和它对应的 method 映射表，现在已经被生成的 schema 取代并删除了 —— 如果哪天需要还是能在
git 历史里找到。

## 技术栈

`prost` + `prost-build` + `protox`，proto3 语法，和 `douyin_live_fetcher` 现有的技术栈
保持一致，让两个项目共用同一套 protobuf 运行时。
