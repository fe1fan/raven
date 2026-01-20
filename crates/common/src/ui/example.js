import { MessageBox } from 'raven/ui/message';

// SwiftUI 风格：链式调用 + 声明式
MessageBox()
    .title("确认删除")
    .message("此操作不可恢复")
    .confirmButton("删除", { style: 'destructive' })
    .cancelButton("取消")
    .show()
    .then(result => {
        if (result.confirmed) {
            console.log("删除");
        }
    });

// 或者更简洁的
const confirmed = await MessageBox()
    .title("Are you sure?")
    .show();

// 快捷方法
MessageBox.confirm("确定删除吗？");
MessageBox.alert("操作成功");
