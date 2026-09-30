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
        ListNode dummy;
        ListNode* p = &dummy;
        int carry = 0;
        while (l1 && l2) {
            l1->val = l1->val + l2->val + carry;
            carry = l1->val / 10;
            l1->val = l1->val % 10;
            p->next = l1;
            l1 = l1->next;
            l2 = l2->next;
            p = p->next;
        }

        if (l1) {
            p->next = l1;
        } else {
            p->next = l2;
        }

        while (p->next) {
            p = p->next;
            p->val += carry;
            carry = p->val / 10;
            p->val %= 10;
        }        

        if (carry) {
            p->next = new ListNode(carry);
        }

        return dummy.next;    
    }
};
