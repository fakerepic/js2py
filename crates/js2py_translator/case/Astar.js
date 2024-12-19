function aStar(graph, start, end, h) {
    var n = graph.length; // 节点数量
    var open = []; // 开放列表，存储 [节点, f值]
    var closed = []; // 关闭列表，标记节点是否已访问
    var g = []; // 存储从起点到每个节点的实际代价 g(x)
    var prev = []; // 存储每个节点的前驱节点，用于重建路径
    var i = 0;

    // 初始化 g 数组和 closed 数组
    while (i < n) {
        g.push(9999999); // 初始代价为无穷大
        closed.push(0); // 所有节点都未访问
        prev.push(-1); // 前驱节点初始化为 -1
        i = i + 1;
    }

    g[start] = 0; // 起点的实际代价为 0
    open.push([start, h[start]]); // 起点加入开放列表

    // 主循环
    while (open.length > 0) {
        // 从开放列表中取出 f 值最小的节点
        var minIndex = 0;
        var j = 1;
        while (j < open.length) {
            if (open[j][1] < open[minIndex][1]) {
                minIndex = j;
            }
            j = j + 1;
        }

        var curr = open[minIndex]; 
        var newOpen = []; 
        var k = 0;
        while (k < open.length) {
            if (k != minIndex) {
                newOpen.push(open[k]);
            }
            k = k + 1;
        }
        open = newOpen;

        var u = curr[0]; // 当前节点

        // 如果当前节点是目标节点，重建路径并返回
        if (u === end) {
            var path = [];
            while (u != -1) {
                path.push(u); 
                u = prev[u];
            }

            // 手动反转路径数组
            var reversedPath = [];
            var p = path.length - 1;
            while (p >= 0) {
                reversedPath.push(path[p]);
                p = p - 1;
            }

            return reversedPath; // 返回从起点到终点的路径
        }

        // 将当前节点标记为已访问
        closed[u] = 1;

        // 遍历当前节点的所有邻居
        var v = 0;
        while (v < n) {
            if (graph[u][v] != 0 && !closed[v]) { // 存在边且未访问
                var newG = g[u] + graph[u][v]; // 计算从起点到 v 的新代价
                if (newG < g[v]) {
                    g[v] = newG; // 更新实际代价
                    prev[v] = u; // 更新前驱节点
                    var f = newG + h[v]; // 计算总代价 f(x)
                    // 如果 v 不在开放列表中，则加入
                    var found = 0;
                    var m = 0;
                    while (m < open.length) {
                        if (open[m][0] === v) {
                            found = 1;
                            open[m][1] = f; // 更新 f 值
                            break;
                        }
                        m = m + 1;
                    }
                    if (!found) {
                        open.push([v, f]);
                    }
                }
            }
            v = v + 1;
        }
    }

    // 如果无法到达目标节点，返回空数组
    return [];
}

// 测试用例
var graph = [
    [0, 1, 4, 0, 0],
    [1, 0, 2, 5, 0],
    [4, 2, 0, 1, 3],
    [0, 5, 1, 0, 2],
    [0, 0, 3, 2, 0]
];

// 启发函数 h(x)（假设为目标节点 4 的曼哈顿距离）
var h = [7, 6, 3, 2, 0]; 

var start = 0; 
var end = 4; 

console.log(aStar(graph, start, end, h));