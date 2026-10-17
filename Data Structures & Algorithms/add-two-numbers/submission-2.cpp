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
    ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
        ListNode dummy(0, l1);
        ListNode* p = &dummy;
        int carry = 0;
        while (l1 && l2) {
            int sum = l1->val + l2->val + carry;
            carry = sum / 10;
            l1->val = sum % 10;
            p = p->next;
            l1 = l1->next;
            l2 = l2->next;
        }
        if (l1==nullptr) {
            p->next = l2;
        } else {
            p->next = l1;
        }
        while (p->next) {
            p = p->next;
            int sum = p->val + carry;
            carry = sum / 10;
            p->val = sum % 10;
        }
        if (carry > 0) {
            p->next = new ListNode(carry);
        }

        return dummy.next;
    }
};
