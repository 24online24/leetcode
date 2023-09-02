#include <iostream>

// Definition for a Node.
class Node {
   public:
    int val;
    Node* prev;
    Node* next;
    Node* child;
};

class Solution {
   private:
    Node* add_node(Node* current) {
    }

   public:
    Node* flatten(Node* head) {
        Node* initialHead = head;
        while (head != nullptr) {
            // std::cout << head->val << ' ';
            if (head->child) {
                Node* topListA = head;
                Node* topListB = head->next;
                Node* bottomListA = head->child;
                Node* bottomListB = head->child;
                while (bottomListB->next) {
                    // std::cout << bottomListB->val << ' ';
                    bottomListB = bottomListB->next;
                    // std::cout << 1 << ' ';
                }
                topListA->next = bottomListA;
                bottomListA->prev = topListA;
                topListB->prev = bottomListB;
                bottomListB->next = topListB;
                head = topListB;
                // std::cout << "Finished adding";
            } else {
                head = head->next;
            }
        }
        Node* testHead = initialHead;
        while (testHead) {
            std::cout << testHead->val << ' ';
            testHead = testHead->next;
        }
        std::cout << "------------------------\n";
        return initialHead;
    }
};

int main() {
    Node* node1 = new Node();
    Node* node2 = new Node();
    Node* node3 = new Node();

    node1->child = node3;
    node1->next = node2;
    node1->val = 1;

    node2->prev = node1;
    node2->val = 2;

    node3->val = 3;

    Solution sol;
    sol.flatten(node1);
}