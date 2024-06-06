#[derive(Debug)]
struct Node {
    element : u32,
    next : next_element,
}

type next_element = Option<Box<Node>>;

#[derive(Debug)]
struct LinkedList {
    head : next_element,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head : None,
        }
    }

    fn add(&mut self , element : u32) {
        let mut current_node = self.head.take();
        let new_node = Node {
            element : element,
            next : current_node,
        };
        self.head = Some(Box::new(new_node));
    }

    fn remove(&mut self) -> Option<u32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }
}

fn main(){
    let mut new_list = LinkedList {
        head : None
    };
    new_list.add(1);
    new_list.add(2);
    new_list.add(3);
    new_list.add(4);
    new_list.remove();
    println!("{:?}" , new_list);
}
