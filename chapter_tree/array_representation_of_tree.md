---
comments: true
---

# 7.3 &nbsp; 二叉树数组表示

在链表表示下，二叉树的存储单元为节点 `TreeNode` ，节点之间通过指针相连接。在上节中，我们学习了在链表表示下的二叉树的各项基本操作。

那么，我们能否用数组来表示二叉树呢？答案是肯定的。

## 7.3.1 &nbsp; 表示完美二叉树

先分析一个简单案例。给定一个完美二叉树，我们将所有节点按照层序遍历的顺序存储在一个数组中，则每个节点都对应唯一的数组索引。

根据层序遍历的特性，我们可以推导出父节点索引与子节点索引之间的“映射公式”：**若节点的索引为 $i$ ，则该节点的左子节点索引为 $2i + 1$ ，右子节点索引为 $2i + 2$** 。下图展示了各个节点索引之间的映射关系。

![完美二叉树的数组表示](array_representation_of_tree.assets/array_representation_binary_tree.png)

<p align="center"> 图：完美二叉树的数组表示 </p>

**映射公式的角色相当于链表中的指针**。给定数组中的任意一个节点，我们都可以通过映射公式来访问它的左（右）子节点。

## 7.3.2 &nbsp; 表示任意二叉树

完美二叉树是一个特例，在二叉树的中间层通常存在许多 $\text{None}$ 。由于层序遍历序列并不包含这些 $\text{None}$ ，因此我们无法仅凭该序列来推测 $\text{None}$ 的数量和分布位置。**这意味着存在多种二叉树结构都符合该层序遍历序列**。

如下图所示，给定一个非完美二叉树，上述的数组表示方法已经失效。

![层序遍历序列对应多种二叉树可能性](array_representation_of_tree.assets/array_representation_without_empty.png)

<p align="center"> 图：层序遍历序列对应多种二叉树可能性 </p>

为了解决此问题，**我们可以考虑在层序遍历序列中显式地写出所有 $\text{None}$** 。如下图所示，这样处理后，层序遍历序列就可以唯一表示二叉树了。

=== "Java"

    ```java title=""
    /* 二叉树的数组表示 */
    // 使用 int 的包装类 Integer ，就可以使用 null 来标记空位
    Integer[] tree = { 1, 2, 3, 4, null, 6, 7, 8, 9, null, null, 12, null, null, 15 };
    ```

=== "C++"

    ```cpp title=""
    /* 二叉树的数组表示 */
    // 使用 int 最大值 INT_MAX 标记空位
    vector<int> tree = {1, 2, 3, 4, INT_MAX, 6, 7, 8, 9, INT_MAX, INT_MAX, 12, INT_MAX, INT_MAX, 15};
    ```

=== "Python"

    ```python title=""
    # 二叉树的数组表示
    # 使用 None 来表示空位
    tree = [1, 2, 3, 4, None, 6, 7, 8, 9, None, None, 12, None, None, 15]
    ```

=== "Go"

    ```go title=""
    /* 二叉树的数组表示 */
    // 使用 any 类型的切片, 就可以使用 nil 来标记空位
    tree := []any{1, 2, 3, 4, nil, 6, 7, 8, 9, nil, nil, 12, nil, nil, 15}
    ```

=== "JS"

    ```javascript title=""
    /* 二叉树的数组表示 */
    // 使用 null 来表示空位
    let tree = [1, 2, 3, 4, null, 6, 7, 8, 9, null, null, 12, null, null, 15];
    ```

=== "TS"

    ```typescript title=""
    /* 二叉树的数组表示 */
    // 使用 null 来表示空位
    let tree: (number | null)[] = [1, 2, 3, 4, null, 6, 7, 8, 9, null, null, 12, null, null, 15];
    ```

=== "C"

    ```c title=""
    /* 二叉树的数组表示 */
    // 使用 int 最大值标记空位，因此要求节点值不能为 INT_MAX
    int tree[] = {1, 2, 3, 4, INT_MAX, 6, 7, 8, 9, INT_MAX, INT_MAX, 12, INT_MAX, INT_MAX, 15};
    ```

=== "C#"

    ```csharp title=""
    /* 二叉树的数组表示 */
    // 使用 int? 可空类型 ，就可以使用 null 来标记空位
    int?[] tree = { 1, 2, 3, 4, null, 6, 7, 8, 9, null, null, 12, null, null, 15 };
    ```

=== "Swift"

    ```swift title=""
    /* 二叉树的数组表示 */
    // 使用 Int? 可空类型 ，就可以使用 nil 来标记空位
    let tree: [Int?] = [1, 2, 3, 4, nil, 6, 7, 8, 9, nil, nil, 12, nil, nil, 15]
    ```

=== "Zig"

    ```zig title=""

    ```

=== "Dart"

    ```dart title=""
    /* 二叉树的数组表示 */
    // 使用 int? 可空类型 ，就可以使用 null 来标记空位
    List<int?> tree = [1, 2, 3, 4, null, 6, 7, 8, 9, null, null, 12, null, null, 15];
    ```

=== "Rust"

    ```rust title=""

    ```

![任意类型二叉树的数组表示](array_representation_of_tree.assets/array_representation_with_empty.png)

<p align="center"> 图：任意类型二叉树的数组表示 </p>

值得说明的是，**完全二叉树非常适合使用数组来表示**。回顾完全二叉树的定义，$\text{None}$ 只出现在最底层且靠右的位置，**因此所有 $\text{None}$ 一定出现在层序遍历序列的末尾**。

这意味着使用数组表示完全二叉树时，可以省略存储所有 $\text{None}$ ，非常方便。下图给出了一个例子。

![完全二叉树的数组表示](array_representation_of_tree.assets/array_representation_complete_binary_tree.png)

<p align="center"> 图：完全二叉树的数组表示 </p>

如下代码给出了数组表示下的二叉树的简单实现，包括以下操作：

- 给定某节点，获取它的值、左（右）子节点、父节点。
- 获取前序遍历、中序遍历、后序遍历、层序遍历序列。

=== "Java"

    ```java title="array_binary_tree.java"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
        private List<Integer> tree;

        /* 构造方法 */
        public ArrayBinaryTree(List<Integer> arr) {
            tree = new ArrayList<>(arr);
        }

        /* 节点数量 */
        public int size() {
            return tree.size();
        }

        /* 获取索引为 i 节点的值 */
        public Integer val(int i) {
            // 若索引越界，则返回 null ，代表空位
            if (i < 0 || i >= size())
                return null;
            return tree.get(i);
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        public Integer left(int i) {
            return 2 * i + 1;
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        public Integer right(int i) {
            return 2 * i + 2;
        }

        /* 获取索引为 i 节点的父节点的索引 */
        public Integer parent(int i) {
            return (i - 1) / 2;
        }

        /* 层序遍历 */
        public List<Integer> levelOrder() {
            List<Integer> res = new ArrayList<>();
            // 直接遍历数组
            for (int i = 0; i < size(); i++) {
                if (val(i) != null)
                    res.add(val(i));
            }
            return res;
        }

        /* 深度优先遍历 */
        private void dfs(Integer i, String order, List<Integer> res) {
            // 若为空位，则返回
            if (val(i) == null)
                return;
            // 前序遍历
            if (order == "pre")
                res.add(val(i));
            dfs(left(i), order, res);
            // 中序遍历
            if (order == "in")
                res.add(val(i));
            dfs(right(i), order, res);
            // 后序遍历
            if (order == "post")
                res.add(val(i));
        }

        /* 前序遍历 */
        public List<Integer> preOrder() {
            List<Integer> res = new ArrayList<>();
            dfs(0, "pre", res);
            return res;
        }

        /* 中序遍历 */
        public List<Integer> inOrder() {
            List<Integer> res = new ArrayList<>();
            dfs(0, "in", res);
            return res;
        }

        /* 后序遍历 */
        public List<Integer> postOrder() {
            List<Integer> res = new ArrayList<>();
            dfs(0, "post", res);
            return res;
        }
    }
    ```

=== "C++"

    ```cpp title="array_binary_tree.cpp"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
      public:
        /* 构造方法 */
        ArrayBinaryTree(vector<int> arr) {
            tree = arr;
        }

        /* 节点数量 */
        int size() {
            return tree.size();
        }

        /* 获取索引为 i 节点的值 */
        int val(int i) {
            // 若索引越界，则返回 INT_MAX ，代表空位
            if (i < 0 || i >= size())
                return INT_MAX;
            return tree[i];
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        int left(int i) {
            return 2 * i + 1;
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        int right(int i) {
            return 2 * i + 2;
        }

        /* 获取索引为 i 节点的父节点的索引 */
        int parent(int i) {
            return (i - 1) / 2;
        }

        /* 层序遍历 */
        vector<int> levelOrder() {
            vector<int> res;
            // 直接遍历数组
            for (int i = 0; i < size(); i++) {
                if (val(i) != INT_MAX)
                    res.push_back(val(i));
            }
            return res;
        }

        /* 前序遍历 */
        vector<int> preOrder() {
            vector<int> res;
            dfs(0, "pre", res);
            return res;
        }

        /* 中序遍历 */
        vector<int> inOrder() {
            vector<int> res;
            dfs(0, "in", res);
            return res;
        }

        /* 后序遍历 */
        vector<int> postOrder() {
            vector<int> res;
            dfs(0, "post", res);
            return res;
        }

      private:
        vector<int> tree;

        /* 深度优先遍历 */
        void dfs(int i, string order, vector<int> &res) {
            // 若为空位，则返回
            if (val(i) == INT_MAX)
                return;
            // 前序遍历
            if (order == "pre")
                res.push_back(val(i));
            dfs(left(i), order, res);
            // 中序遍历
            if (order == "in")
                res.push_back(val(i));
            dfs(right(i), order, res);
            // 后序遍历
            if (order == "post")
                res.push_back(val(i));
        }
    };
    ```

=== "Python"

    ```python title="array_binary_tree.py"
    class ArrayBinaryTree:
        """数组表示下的二叉树类"""

        def __init__(self, arr: list[int | None]):
            """构造方法"""
            self.__tree = list(arr)

        def size(self):
            """节点数量"""
            return len(self.__tree)

        def val(self, i: int) -> int:
            """获取索引为 i 节点的值"""
            # 若索引越界，则返回 None ，代表空位
            if i < 0 or i >= self.size():
                return None
            return self.__tree[i]

        def left(self, i: int) -> int | None:
            """获取索引为 i 节点的左子节点的索引"""
            return 2 * i + 1

        def right(self, i: int) -> int | None:
            """获取索引为 i 节点的右子节点的索引"""
            return 2 * i + 2

        def parent(self, i: int) -> int | None:
            """获取索引为 i 节点的父节点的索引"""
            return (i - 1) // 2

        def level_order(self) -> list[int]:
            """层序遍历"""
            self.res = []
            # 直接遍历数组
            for i in range(self.size()):
                if self.val(i) is not None:
                    self.res.append(self.val(i))
            return self.res

        def __dfs(self, i: int, order: str):
            """深度优先遍历"""
            if self.val(i) is None:
                return
            # 前序遍历
            if order == "pre":
                self.res.append(self.val(i))
            self.__dfs(self.left(i), order)
            # 中序遍历
            if order == "in":
                self.res.append(self.val(i))
            self.__dfs(self.right(i), order)
            # 后序遍历
            if order == "post":
                self.res.append(self.val(i))

        def pre_order(self) -> list[int]:
            """前序遍历"""
            self.res = []
            self.__dfs(0, order="pre")
            return self.res

        def in_order(self) -> list[int]:
            """中序遍历"""
            self.res = []
            self.__dfs(0, order="in")
            return self.res

        def post_order(self) -> list[int]:
            """后序遍历"""
            self.res = []
            self.__dfs(0, order="post")
            return self.res
    ```

=== "Go"

    ```go title="array_binary_tree.go"
    /* 数组表示下的二叉树类 */
    type arrayBinaryTree struct {
        tree []any
    }

    /* 构造方法 */
    func newArrayBinaryTree(arr []any) *arrayBinaryTree {
        return &arrayBinaryTree{
            tree: arr,
        }
    }

    /* 节点数量 */
    func (abt *arrayBinaryTree) size() int {
        return len(abt.tree)
    }

    /* 获取索引为 i 节点的值 */
    func (abt *arrayBinaryTree) val(i int) any {
        // 若索引越界，则返回 null ，代表空位
        if i < 0 || i >= abt.size() {
            return nil
        }
        return abt.tree[i]
    }

    /* 获取索引为 i 节点的左子节点的索引 */
    func (abt *arrayBinaryTree) left(i int) int {
        return 2*i + 1
    }

    /* 获取索引为 i 节点的右子节点的索引 */
    func (abt *arrayBinaryTree) right(i int) int {
        return 2*i + 2
    }

    /* 获取索引为 i 节点的父节点的索引 */
    func (abt *arrayBinaryTree) parent(i int) int {
        return (i - 1) / 2
    }

    /* 层序遍历 */
    func (abt *arrayBinaryTree) levelOrder() []any {
        var res []any
        // 直接遍历数组
        for i := 0; i < abt.size(); i++ {
            if abt.val(i) != nil {
                res = append(res, abt.val(i))
            }
        }
        return res
    }

    /* 深度优先遍历 */
    func (abt *arrayBinaryTree) dfs(i int, order string, res *[]any) {
        // 若为空位，则返回
        if abt.val(i) == nil {
            return
        }
        // 前序遍历
        if order == "pre" {
            *res = append(*res, abt.val(i))
        }
        abt.dfs(abt.left(i), order, res)
        // 中序遍历
        if order == "in" {
            *res = append(*res, abt.val(i))
        }
        abt.dfs(abt.right(i), order, res)
        // 后序遍历
        if order == "post" {
            *res = append(*res, abt.val(i))
        }
    }

    /* 前序遍历 */
    func (abt *arrayBinaryTree) preOrder() []any {
        var res []any
        abt.dfs(0, "pre", &res)
        return res
    }

    /* 中序遍历 */
    func (abt *arrayBinaryTree) inOrder() []any {
        var res []any
        abt.dfs(0, "in", &res)
        return res
    }

    /* 后序遍历 */
    func (abt *arrayBinaryTree) postOrder() []any {
        var res []any
        abt.dfs(0, "post", &res)
        return res
    }
    ```

=== "JS"

    ```javascript title="array_binary_tree.js"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
        #tree;

        /* 构造方法 */
        constructor(arr) {
            this.#tree = arr;
        }

        /* 节点数量 */
        size() {
            return this.#tree.length;
        }

        /* 获取索引为 i 节点的值 */
        val(i) {
            // 若索引越界，则返回 null ，代表空位
            if (i < 0 || i >= this.size()) return null;
            return this.#tree[i];
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        left(i) {
            return 2 * i + 1;
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        right(i) {
            return 2 * i + 2;
        }

        /* 获取索引为 i 节点的父节点的索引 */
        parent(i) {
            return (i - 1) / 2;
        }

        /* 层序遍历 */
        levelOrder() {
            let res = [];
            // 直接遍历数组
            for (let i = 0; i < this.size(); i++) {
                if (this.val(i) !== null) res.push(this.val(i));
            }
            return res;
        }

        /* 深度优先遍历 */
        #dfs(i, order, res) {
            // 若为空位，则返回
            if (this.val(i) === null) return;
            // 前序遍历
            if (order === 'pre') res.push(this.val(i));
            this.#dfs(this.left(i), order, res);
            // 中序遍历
            if (order === 'in') res.push(this.val(i));
            this.#dfs(this.right(i), order, res);
            // 后序遍历
            if (order === 'post') res.push(this.val(i));
        }

        /* 前序遍历 */
        preOrder() {
            const res = [];
            this.#dfs(0, 'pre', res);
            return res;
        }

        /* 中序遍历 */
        inOrder() {
            const res = [];
            this.#dfs(0, 'in', res);
            return res;
        }

        /* 后序遍历 */
        postOrder() {
            const res = [];
            this.#dfs(0, 'post', res);
            return res;
        }
    }
    ```

=== "TS"

    ```typescript title="array_binary_tree.ts"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
        #tree: (number | null)[];

        /* 构造方法 */
        constructor(arr: (number | null)[]) {
            this.#tree = arr;
        }

        /* 节点数量 */
        size(): number {
            return this.#tree.length;
        }

        /* 获取索引为 i 节点的值 */
        val(i: number): number | null {
            // 若索引越界，则返回 null ，代表空位
            if (i < 0 || i >= this.size()) return null;
            return this.#tree[i];
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        left(i: number): number {
            return 2 * i + 1;
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        right(i: number): number {
            return 2 * i + 2;
        }

        /* 获取索引为 i 节点的父节点的索引 */
        parent(i: number): number {
            return (i - 1) / 2;
        }

        /* 层序遍历 */
        levelOrder(): number[] {
            let res = [];
            // 直接遍历数组
            for (let i = 0; i < this.size(); i++) {
                if (this.val(i) !== null) res.push(this.val(i));
            }
            return res;
        }

        /* 深度优先遍历 */
        #dfs(i: number, order: Order, res: (number | null)[]): void {
            // 若为空位，则返回
            if (this.val(i) === null) return;
            // 前序遍历
            if (order === 'pre') res.push(this.val(i));
            this.#dfs(this.left(i), order, res);
            // 中序遍历
            if (order === 'in') res.push(this.val(i));
            this.#dfs(this.right(i), order, res);
            // 后序遍历
            if (order === 'post') res.push(this.val(i));
        }

        /* 前序遍历 */
        preOrder(): (number | null)[] {
            const res = [];
            this.#dfs(0, 'pre', res);
            return res;
        }

        /* 中序遍历 */
        inOrder(): (number | null)[] {
            const res = [];
            this.#dfs(0, 'in', res);
            return res;
        }

        /* 后序遍历 */
        postOrder(): (number | null)[] {
            const res = [];
            this.#dfs(0, 'post', res);
            return res;
        }
    }
    ```

=== "C"

    ```c title="array_binary_tree.c"
    /* 数组表示下的二叉树类 */
    struct arrayBinaryTree {
        vector *tree;
    };

    typedef struct arrayBinaryTree arrayBinaryTree;

    /* 构造函数 */
    arrayBinaryTree *newArrayBinaryTree(vector *arr) {
        arrayBinaryTree *newABT = malloc(sizeof(arrayBinaryTree));
        newABT->tree = arr;
        return newABT;
    }

    /* 节点数量 */
    int size(arrayBinaryTree *abt) {
        return abt->tree->size;
    }

    /* 获取索引为 i 节点的值 */
    int val(arrayBinaryTree *abt, int i) {
        // 若索引越界，则返回 INT_MAX ，代表空位
        if (i < 0 || i >= size(abt))
            return INT_MAX;
        return *(int *)abt->tree->data[i];
    }

    /* 深度优先遍历 */
    void dfs(arrayBinaryTree *abt, int i, const char *order, vector *res) {
        // 若为空位，则返回
        if (val(abt, i) == INT_MAX)
            return;
        // 前序遍历
        if (strcmp(order, "pre") == 0) {
            int tmp = val(abt, i);
            vectorPushback(res, &tmp, sizeof(tmp));
        }
        dfs(abt, left(i), order, res);
        // 中序遍历
        if (strcmp(order, "in") == 0) {
            int tmp = val(abt, i);
            vectorPushback(res, &tmp, sizeof(tmp));
        }
        dfs(abt, right(i), order, res);
        // 后序遍历
        if (strcmp(order, "post") == 0) {
            int tmp = val(abt, i);
            vectorPushback(res, &tmp, sizeof(tmp));
        }
    }

    /* 层序遍历 */
    vector *levelOrder(arrayBinaryTree *abt) {
        vector *res = newVector();
        // 直接遍历数组
        for (int i = 0; i < size(abt); i++) {
            if (val(abt, i) != INT_MAX) {
                int tmp = val(abt, i);
                vectorPushback(res, &tmp, sizeof(int));
            }
        }
        return res;
    }

    /* 前序遍历 */
    vector *preOrder(arrayBinaryTree *abt) {
        vector *res = newVector();
        dfs(abt, 0, "pre", res);
        return res;
    }

    /* 中序遍历 */
    vector *inOrder(arrayBinaryTree *abt) {
        vector *res = newVector();
        dfs(abt, 0, "in", res);
        return res;
    }

    /* 后序遍历 */
    vector *postOrder(arrayBinaryTree *abt) {
        vector *res = newVector();
        dfs(abt, 0, "post", res);
        return res;
    }
    ```

=== "C#"

    ```csharp title="array_binary_tree.cs"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
        private List<int?> tree;

        /* 构造方法 */
        public ArrayBinaryTree(List<int?> arr) {
            tree = new List<int?>(arr);
        }

        /* 节点数量 */
        public int size() {
            return tree.Count;
        }

        /* 获取索引为 i 节点的值 */
        public int? val(int i) {
            // 若索引越界，则返回 null ，代表空位
            if (i < 0 || i >= size())
                return null;
            return tree[i];
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        public int left(int i) {
            return 2 * i + 1;
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        public int right(int i) {
            return 2 * i + 2;
        }

        /* 获取索引为 i 节点的父节点的索引 */
        public int parent(int i) {
            return (i - 1) / 2;
        }

        /* 层序遍历 */
        public List<int> levelOrder() {
            List<int> res = new List<int>();
            // 直接遍历数组
            for (int i = 0; i < size(); i++) {
                if (val(i).HasValue)
                    res.Add(val(i).Value);
            }
            return res;
        }

        /* 深度优先遍历 */
        private void dfs(int i, string order, List<int> res) {
            // 若为空位，则返回
            if (!val(i).HasValue)
                return;
            // 前序遍历
            if (order == "pre")
                res.Add(val(i).Value);
            dfs(left(i), order, res);
            // 中序遍历
            if (order == "in")
                res.Add(val(i).Value);
            dfs(right(i), order, res);
            // 后序遍历
            if (order == "post")
                res.Add(val(i).Value);
        }

        /* 前序遍历 */
        public List<int> preOrder() {
            List<int> res = new List<int>();
            dfs(0, "pre", res);
            return res;
        }

        /* 中序遍历 */
        public List<int> inOrder() {
            List<int> res = new List<int>();
            dfs(0, "in", res);
            return res;
        }

        /* 后序遍历 */
        public List<int> postOrder() {
            List<int> res = new List<int>();
            dfs(0, "post", res);
            return res;
        }
    }
    ```

=== "Swift"

    ```swift title="array_binary_tree.swift"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
        private var tree: [Int?]

        /* 构造方法 */
        init(arr: [Int?]) {
            tree = arr
        }

        /* 节点数量 */
        func size() -> Int {
            tree.count
        }

        /* 获取索引为 i 节点的值 */
        func val(i: Int) -> Int? {
            // 若索引越界，则返回 null ，代表空位
            if i < 0 || i >= size() {
                return nil
            }
            return tree[i]
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        func left(i: Int) -> Int {
            2 * i + 1
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        func right(i: Int) -> Int {
            2 * i + 2
        }

        /* 获取索引为 i 节点的父节点的索引 */
        func parent(i: Int) -> Int {
            (i - 1) / 2
        }

        /* 层序遍历 */
        func levelOrder() -> [Int] {
            var res: [Int] = []
            // 直接遍历数组
            for i in stride(from: 0, to: size(), by: 1) {
                if let val = val(i: i) {
                    res.append(val)
                }
            }
            return res
        }

        /* 深度优先遍历 */
        private func dfs(i: Int, order: String, res: inout [Int]) {
            // 若为空位，则返回
            guard let val = val(i: i) else {
                return
            }
            // 前序遍历
            if order == "pre" {
                res.append(val)
            }
            dfs(i: left(i: i), order: order, res: &res)
            // 中序遍历
            if order == "in" {
                res.append(val)
            }
            dfs(i: right(i: i), order: order, res: &res)
            // 后序遍历
            if order == "post" {
                res.append(val)
            }
        }

        /* 前序遍历 */
        func preOrder() -> [Int] {
            var res: [Int] = []
            dfs(i: 0, order: "pre", res: &res)
            return res
        }

        /* 中序遍历 */
        func inOrder() -> [Int] {
            var res: [Int] = []
            dfs(i: 0, order: "in", res: &res)
            return res
        }

        /* 后序遍历 */
        func postOrder() -> [Int] {
            var res: [Int] = []
            dfs(i: 0, order: "post", res: &res)
            return res
        }
    }
    ```

=== "Zig"

    ```zig title="array_binary_tree.zig"
    [class]{ArrayBinaryTree}-[func]{}
    ```

=== "Dart"

    ```dart title="array_binary_tree.dart"
    /* 数组表示下的二叉树类 */
    class ArrayBinaryTree {
      late List<int?> _tree;

      /* 构造方法 */
      ArrayBinaryTree(this._tree);

      /* 节点数量 */
      int size() {
        return _tree.length;
      }

      /* 获取索引为 i 节点的值 */
      int? val(int i) {
        // 若索引越界，则返回 null ，代表空位
        if (i < 0 || i >= size()) {
          return null;
        }
        return _tree[i];
      }

      /* 获取索引为 i 节点的左子节点的索引 */
      int? left(int i) {
        return 2 * i + 1;
      }

      /* 获取索引为 i 节点的右子节点的索引 */
      int? right(int i) {
        return 2 * i + 2;
      }

      /* 获取索引为 i 节点的父节点的索引 */
      int? parent(int i) {
        return (i - 1) ~/ 2;
      }

      /* 层序遍历 */
      List<int> levelOrder() {
        List<int> res = [];
        for (int i = 0; i < size(); i++) {
          if (val(i) != null) {
            res.add(val(i)!);
          }
        }
        return res;
      }

      /* 深度优先遍历 */
      void dfs(int i, String order, List<int?> res) {
        // 若为空位，则返回
        if (val(i) == null) {
          return;
        }
        // 前序遍历
        if (order == 'pre') {
          res.add(val(i));
        }
        dfs(left(i)!, order, res);
        // 中序遍历
        if (order == 'in') {
          res.add(val(i));
        }
        dfs(right(i)!, order, res);
        // 后序遍历
        if (order == 'post') {
          res.add(val(i));
        }
      }

      /* 前序遍历 */
      List<int?> preOrder() {
        List<int?> res = [];
        dfs(0, 'pre', res);
        return res;
      }

      /* 中序遍历 */
      List<int?> inOrder() {
        List<int?> res = [];
        dfs(0, 'in', res);
        return res;
      }

      /* 后序遍历 */
      List<int?> postOrder() {
        List<int?> res = [];
        dfs(0, 'post', res);
        return res;
      }
    }
    ```

=== "Rust"

    ```rust title="array_binary_tree.rs"
    /* 数组表示下的二叉树类 */
    struct ArrayBinaryTree {
        tree: Vec<Option<i32>>,
    }

    impl ArrayBinaryTree {
        /* 构造方法 */
        fn new(arr: Vec<Option<i32>>) -> Self {
            Self { tree: arr }
        }

        /* 节点数量 */
        fn size(&self) -> i32 {
            self.tree.len() as i32
        }

        /* 获取索引为 i 节点的值 */
        fn val(&self, i: i32) -> Option<i32> {
            // 若索引越界，则返回 None ，代表空位
            if i < 0 || i >= self.size() {
                None
            } else {
                self.tree[i as usize]
            }
        }

        /* 获取索引为 i 节点的左子节点的索引 */
        fn left(&self, i: i32) -> i32 {
            2 * i + 1
        }

        /* 获取索引为 i 节点的右子节点的索引 */
        fn right(&self, i: i32) -> i32 {
            2 * i + 2
        }

        /* 获取索引为 i 节点的父节点的索引 */
        fn parent(&self, i: i32) -> i32 {
            (i - 1) / 2
        }

        /* 层序遍历 */
        fn level_order(&self) -> Vec<i32> {
            let mut res = vec![];
            // 直接遍历数组
            for i in 0..self.size() {
                if let Some(val) = self.val(i) {
                    res.push(val)
                }
            }
            res
        }

        /* 深度优先遍历 */
        fn dfs(&self, i: i32, order: &str, res: &mut Vec<i32>) {
            if self.val(i).is_none() {
                return;
            }
            let val = self.val(i).unwrap();
            // 前序遍历
            if order == "pre" {
                res.push(val);
            }
            self.dfs(self.left(i), order, res);
            // 中序遍历
            if order == "in" {
                res.push(val);
            }
            self.dfs(self.right(i), order, res);
            // 后序遍历
            if order == "post" {
                res.push(val);
            }
        }

        /* 前序遍历 */
        fn pre_order(&self) -> Vec<i32> {
            let mut res = vec![];
            self.dfs(0, "pre", &mut res);
            res
        }

        /* 中序遍历 */
        fn in_order(&self) -> Vec<i32> {
            let mut res = vec![];
            self.dfs(0, "in", &mut res);
            res
        }

        /* 后序遍历 */
        fn post_order(&self) -> Vec<i32> {
            let mut res = vec![];
            self.dfs(0, "post", &mut res);
            res
        }
    }
    ```

## 7.3.3 &nbsp; 优势与局限性

二叉树的数组表示的优点包括：

- 数组存储在连续的内存空间中，对缓存友好，访问与遍历速度较快。
- 不需要存储指针，比较节省空间。
- 允许随机访问节点。

然而，数组表示也具有一些局限性：

- 数组存储需要连续内存空间，因此不适合存储数据量过大的树。
- 增删节点需要通过数组插入与删除操作实现，效率较低。
- 当二叉树中存在大量 $\text{None}$ 时，数组中包含的节点数据比重较低，空间利用率较低。