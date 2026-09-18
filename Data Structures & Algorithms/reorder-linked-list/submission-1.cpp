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
        if (!head) return;
        ListNode dummy(0, head);
        ListNode* slow = &dummy;
        ListNode* fast = &dummy;
        while (fast != nullptr && fast->next != nullptr) {
            fast = fast->next->next;
            slow = slow->next;
        }
        ListNode* tail = slow->next;
        slow->next = nullptr;
        fast = reverseList(tail);
        slow = head;
        ListNode* p = &dummy;
        while (slow && fast) {
            p->next = slow;
            p = p->next;
            slow = slow->next;

            p->next = fast;
            fast = fast->next;
            p = p->next;
        }
        if (slow) {
            p->next = slow;
        } else {
            p->next = fast;
        }
        head = dummy.next;
    }

    ListNode* reverseList(ListNode* head) {
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
