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
    void reorderList(ListNode* head) {
        if (head == nullptr) return;
        ListNode* p = head;
        for (int i = 0; i < count(head) / 2; ++i) {
            p = p->next;
        }
        ListNode* q = reverse(p->next);
        p->next = nullptr;
        p = head;
        while (p && q) {
            ListNode* l = p->next;
            p->next = q;

            ListNode* r = q->next;
            q->next = l;

            p = l;
            q = r;
        }
    }

    int count(ListNode* head) {
        int n = 0;
        while (head != nullptr) {
            n++;
            head = head->next;
        }
        return n;
    }

    ListNode* reverse(ListNode* head) {
        ListNode* p = nullptr;
        ListNode* q = head;
        while (q != nullptr) {
            ListNode* r = q->next;
            q->next = p;
            p = q;
            q = r;
        }
        return p;
    }
};
