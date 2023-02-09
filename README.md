# nodes
## Task
Please provide a Rust code for the following statement.

Problem Statement:
- System has the following entities
- Groups
- Nodes

     Let's assume the system has 10 groups; each group can hold multiple nodes under it
      For Example:
        - G1:
            - Node 1
            - Node 2
            - Node 3
       
        - G10:
            - Node 12
            - Node 23
            - Node 35

    - We should be able to add nodes to the system, such that the system will randomly assign a     group to join the node
    - Condition is such that groups should have equal numbers of nodes assigned to them at every time
        - G1:
            - Node 1
            - Node 2
            - Node 3
        
        - G2:
            - Node 3
            - Node 4
            - Node 5

        - G3:
            - Node 6
            - Node 7
            - Node 8
