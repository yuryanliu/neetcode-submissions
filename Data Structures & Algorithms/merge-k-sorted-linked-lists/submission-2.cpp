/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        ListNode dummy;
        auto cmp = [](ListNode* left, ListNode* right) { return left->val > right->val; };
        std::priority_queue<ListNode*, std::vector<ListNode*>, decltype(cmp)> min_heap(cmp);
        for (auto& l : lists) {
            if (l != nullptr) {
                min_heap.push(l);
            }
        }
        ListNode* p = &dummy;
        while (!min_heap.empty()) {
            ListNode* l = min_heap.top(); min_heap.pop();
            p->next = l;
            p = p->next;
            if (l->next != nullptr) {
                min_heap.push(l->next);
            }
        }

        return dummy.next;
    }
};
