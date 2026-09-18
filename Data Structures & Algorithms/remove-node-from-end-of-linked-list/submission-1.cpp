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
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode dummy(0, head);
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast && n > 0) {
            fast = fast->next;
            n--;
        }
        ListNode* prev = &dummy;
        while (slow && fast) {
            prev = slow;
            slow = slow->next;
            fast = fast->next;
        }
        if (slow) {
            prev->next = slow->next;
        }
        return dummy.next;
    }
};
