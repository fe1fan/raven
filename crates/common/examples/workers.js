// 导入需要的绑定 - 它们会被注入到全局作用域
import { KV } from 'raven/kv'
import { UTILS } from 'raven/utils'

export default {
    fetch(request, env, ctx) {
        var url = request.url;
        var method = request.method;

        console.log("收到请求:", method, url);

        // 测试 1: 基本响应
        if (url.indexOf("/hello") !== -1) {
            return new Response("Hello, World!", {
                status: 200,
                headers: { "Content-Type": "text/plain; charset=utf-8" }
            });
        }

        // 测试 2: 直接使用 KV（全局作用域，不需要 env.KV）
        if (url.indexOf("/test-kv") !== -1) {
            try {
                console.log("测试全局 KV 绑定...");
                
                // 直接使用 KV，不需要 env.KV
                KV.put("test-key", "Hello from global KV!");
                console.log("✓ KV.put 成功");
                
                var value = KV.get("test-key");
                console.log("✓ KV.get 成功:", value);
                
                KV.put("counter", "42");
                var counter = KV.get("counter");
                
                return new Response(
                    "KV 全局绑定测试成功!\n\n" +
                    "使用方式: 直接调用 KV.put() 和 KV.get()\n" +
                    "不需要: env.KV.put()\n\n" +
                    "test-key = " + value + "\n" +
                    "counter = " + counter,
                    {
                        status: 200,
                        headers: { "Content-Type": "text/plain; charset=utf-8" }
                    }
                );
            } catch (e) {
                console.error("KV 测试失败:", e);
                return new Response("KV 测试失败: " + e, { status: 500 });
            }
        }

        // 测试 3: 直接使用 UTILS（全局作用域）
        if (url.indexOf("/test-utils") !== -1) {
            try {
                console.log("测试全局 UTILS 绑定...");
                
                // 直接使用 UTILS，不需要 env.UTILS
                var reversed = UTILS.reverse("Hello");
                console.log("✓ UTILS.reverse 成功:", reversed);
                
                var encoded = UTILS.base64Encode("hello world");
                console.log("✓ UTILS.base64Encode 成功:", encoded);
                
                var decoded = UTILS.base64Decode(encoded);
                console.log("✓ UTILS.base64Decode 成功:", decoded);
                
                var hash = UTILS.hash("test data");
                console.log("✓ UTILS.hash 成功");
                
                var sum = UTILS.sum([1, 2, 3, 4, 5]);
                console.log("✓ UTILS.sum 成功:", sum);
                
                var avg = UTILS.average([10, 20, 30]);
                console.log("✓ UTILS.average 成功:", avg);
                
                var ts = UTILS.timestamp();
                console.log("✓ UTILS.timestamp 成功:", ts);
                
                return new Response(
                    "UTILS 全局绑定测试成功!\n\n" +
                    "使用方式: 直接调用 UTILS.reverse() 等\n" +
                    "不需要: env.UTILS.reverse()\n\n" +
                    "reverse('Hello') = " + reversed + "\n" +
                    "base64Encode('hello world') = " + encoded + "\n" +
                    "base64Decode(encoded) = " + decoded + "\n" +
                    "hash('test data') = " + hash.substring(0, 16) + "...\n" +
                    "sum([1,2,3,4,5]) = " + sum + "\n" +
                    "average([10,20,30]) = " + avg + "\n" +
                    "timestamp() = " + ts,
                    {
                        status: 200,
                        headers: { "Content-Type": "text/plain; charset=utf-8" }
                    }
                );
            } catch (e) {
                console.error("UTILS 测试失败:", e);
                return new Response("UTILS 测试失败: " + e, { status: 500 });
            }
        }

        // 测试 4: 组合使用全局绑定
        if (url.indexOf("/test-combo") !== -1) {
            try {
                console.log("测试组合全局绑定...");
                
                // 直接使用全局绑定，代码更简洁
                var data = "hello world";
                var encoded = UTILS.base64Encode(data);
                KV.put("processed-data", encoded);
                
                var stored = KV.get("processed-data");
                var decoded = UTILS.base64Decode(stored);
                var reversed = UTILS.reverse(decoded);
                
                var hash = UTILS.hash(data);
                KV.put("data-hash", hash);
                
                return new Response(
                    "组合测试成功!\n\n" +
                    "原始: " + data + "\n" +
                    "Base64编码: " + encoded + "\n" +
                    "存储后读取: " + stored + "\n" +
                    "解码: " + decoded + "\n" +
                    "反转: " + reversed + "\n" +
                    "哈希: " + hash.substring(0, 16) + "...",
                    {
                        status: 200,
                        headers: { "Content-Type": "text/plain; charset=utf-8" }
                    }
                );
            } catch (e) {
                console.error("组合测试失败:", e);
                return new Response("组合测试失败: " + e, { status: 500 });
            }
        }

        // 测试 5: 检查绑定可用性
        if (url.indexOf("/test-no-import") !== -1) {
            try {
                console.log("测试绑定可用性...");
                
                // 检查全局作用域中的绑定
                var hasKV = typeof KV !== 'undefined';
                var hasUTILS = typeof UTILS !== 'undefined';
                var hasDB = typeof DB !== 'undefined';
                
                // 注意：env 对象现在是空的
                var envHasKV = typeof env.KV !== 'undefined';
                var envHasUTILS = typeof env.UTILS !== 'undefined';
                
                return new Response(
                    "绑定可用性测试:\n\n" +
                    "全局作用域:\n" +
                    "  KV:    " + (hasKV ? "✓ 可用" : "✗ 不可用") + "\n" +
                    "  UTILS: " + (hasUTILS ? "✓ 可用" : "✗ 不可用") + "\n" +
                    "  DB:    " + (hasDB ? "✓ 可用" : "✗ 不可用 (未导入)") + "\n\n" +
                    "env 对象 (已废弃):\n" +
                    "  env.KV:    " + (envHasKV ? "✓ 可用" : "✗ 不可用") + "\n" +
                    "  env.UTILS: " + (envHasUTILS ? "✓ 可用" : "✗ 不可用"),
                    {
                        status: 200,
                        headers: { "Content-Type": "text/plain; charset=utf-8" }
                    }
                );
            } catch (e) {
                console.error("测试失败:", e);
                return new Response("测试失败: " + e, { status: 500 });
            }
        }

        // 默认响应
        return new Response(
            "🔧 全局绑定测试路由:\n\n" +
            "- /hello           (基本响应)\n" +
            "- /test-kv         (测试全局 KV 绑定)\n" +
            "- /test-utils      (测试全局 UTILS 绑定)\n" +
            "- /test-combo      (测试组合功能)\n" +
            "- /test-no-import  (测试绑定可用性)\n\n" +
            "特性: 导入的绑定直接在全局作用域中可用\n" +
            "使用 KV.put() 而不是 env.KV.put()",
            {
                status: 200,
                headers: { "Content-Type": "text/plain; charset=utf-8" }
            }
        );
    }
}
