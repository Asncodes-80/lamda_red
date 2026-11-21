# Drizzle VS Code 扩展

Drizzle 为 VS Code 中的 `.zz`、`.drizzle` 和 `.rd` 文件提供语法高亮。

![Drizzle](../../images/logo_drizzle_language.png)

## 扩展演示

![扩展演示](../../images/drizzle_demo.png)

## 列车控制示例

```drizzle
&& Train control requirements
Estimate @0,0 = /Accurate estimate\nof speed & position/
Tracking @1,0 = <Tracking system>
Train @2,0 = [Train]
Duty @0,1 = {Train controller responsibility\nCompute safe acceleration for all trains between this station and the next.}
Safe @0,2 = /Safe command\nmessage/
Controller @1,2 = <Train controller>
Timing @0,3 = /Command in time/
Send @1,3 = (Send\ncommand)
Command @2,3 = [Command]

Estimate __ Tracking [o]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train __> Controller : As monitor\nmeasuredSpeed\nmeasuredLocation
Controller __> Command : As control\ncommandedSpeed\ncommandedAccel
Safe __ Controller [o]
Timing __ Controller [o] : Responsibility
Controller __ Send [o] : Performance
Duty --> Controller
```

## 编译输出

![编译输出](../../output.png)

图形类型、连线标记和布局选项见 [Drizzle 语法指南](./README.md#drizzle-syntax)。

## 开发与测试

在 VS Code 中打开 `drizzle/` 文件夹。`package.json` 是扩展清单，指定语法文件的位置。`syntaxes/drizzle.tmLanguage.json` 定义 TextMate 语法，`language-configuration.json` 配置注释和括号。

按 **F5** 打开 Extension Development Host，在该窗口打开示例文件，检查语法高亮和语言配置。修改后可重新启动宿主，或重新加载窗口。

可从 Marketplace 安装扩展，也可在 VS Code 中使用 **Extensions: Install from VSIX...** 安装 VSIX 包。开发与发布说明见 [VS Code 扩展文档](https://code.visualstudio.com/api)。

## 扩展发行说明

### 1.1.3

- 修正 Marketplace 中标志、语法演示和渲染结果的图片地址。
- 直接链接仓库 README 中完整的 Drizzle 指南。

### 1.1.0 – 1.1.2

- 扩展 Pest 语法，支持命名节点、前向引用、UTF-8 标签、转义、换行及带行列位置的诊断。保留匿名目标、风险、参与者声明和整行注释。
- 新增对象、动作、注记、云形注释、描述性梯形和菱形。需求和假设等叶节点可用粗边框。
- 新增实线和虚线、单端或双端可选箭头、标签、自环和共享细化汇合点。支持空心圆、实心圆、冲突锯齿和障碍横杠。
- 新增四种布局方向、显式行列定位、直角走线和外围绕行。分离平行连线，将箭头连接到图形边界。
- 更新 VS Code 语法和扩展 README，将 GIF 演示替换为 PNG，并加入 Drizzle 1.1.0 VSIX。

### 1.0.0

Drizzle 首次发布。
