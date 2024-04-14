use std::{cell::RefCell, fmt::Debug, path::Iter, rc::Rc};

#[derive(Debug, PartialEq)]
enum ActionType {
    Increase,
    Decrease,
}

///
/// This oughts to be an iterator of Strings that can be formatted to 
/// "Asset/Current Asset/Cash, Cash Equivalents & Short Term Investments/Cash And Cash Equivalents"
/// from ["Assets", "Current Assets", "Cash, Cash Equivalents & Short Term Investments", "Cash And Cash Equivalents"]
/// 
#[derive(Debug, PartialEq)]
struct PrimaryAccountType {
    name: String,
    on_debit: ActionType,
    on_credit: ActionType,
}

impl PrimaryAccountType {
    fn new(name: &str, on_debit: ActionType, on_credit: ActionType) -> Self {
        // Ascertain on_increase isn't the same as on_decrease
        if on_debit == on_credit {
            println!("Invalid account actions set. on_debit:ActionType = {:?} == on_credit:ActionType = {:?}!", on_debit, on_credit);
        }

        PrimaryAccountType {
            name: name.to_owned(), on_debit, on_credit
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn on_debit(&self) -> &ActionType {
        &self.on_debit
    }

    pub fn on_credit(&self) -> &ActionType {
        &self.on_credit
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn set_action_type(&mut self, on_debit: ActionType, on_credit: ActionType) {
        if on_debit == on_credit {
            println!("Invalid account actions set. on_debit:ActionType = {:?} == on_credit:ActionType = {:?}!", on_debit, on_credit);
            return;
        }

        self.on_debit = on_debit;
        self.on_credit = on_credit;
    }
}

///
/// `AccountTreeNode` trait used to build and account's relational tree
/// Any node that's present on the account's tree is required to implement this trait
///
trait AccountTreeNode {
    ///
    ///  Used to retrieve the level of a node
    /// 
    fn level(&self) -> usize;

    ///
    ///  Used to set the level of a node
    /// 
    fn set_level(&mut self, level: usize);

    ///
    /// Used to set the name of the TreeNode 
    /// This name is the same as the account's name represented by this tree node
    /// 
    fn set_name(&mut self, name: &str);

    ///
    /// Used to get the name of this tree node.
    /// This is the name of the account associated with this tree node
    /// 
    fn name(&self) -> &str;

    ///
    /// Used to get the account_type of this tree node
    ///
    fn account_type(&self) -> &Option<Rc<PrimaryAccountType>>;

    ///
    ///  Used to set a child node's parent's
    /// 
    fn set_parent(&mut self, parent: Option<Rc<RefCell<dyn ParentNode>>>);

    ///
    ///  Use to get the child node's parent
    /// 
    fn parent(&self) -> &Option<Rc<RefCell<dyn ParentNode>>>;
    
}

impl Debug for dyn AccountTreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name: {:?}, level: {:?}, account_type: {:?}, parent: {:?}}}",
            self.name(), self.level(), self.account_type(), self.parent())
    }
}

///
/// `ParentNodeT` trait used to identify certain nodes as parents
/// 
trait ParentNodeT {
    // Used to add a child to the parent node
    fn add_child(&mut self, child: Rc<RefCell<dyn ParentNode>>);

    // Used to get the children of the parent node
    fn children(&self) -> &Vec<Rc<RefCell<dyn ParentNode>>>;
}

///
/// `ChildNodeT` used to identify a node as a child
/// 
trait ChildNodeT {
    // Used to set a child node's parent's
    fn set_parent(&mut self, parent: Rc<RefCell<dyn ParentNode>>);

    // Use to get the child node's parent
    fn parent(&self) -> &RefCell<dyn ParentNode>;
}


///
/// `ParentNode` trait that marks a node as being able to have child nodes under it
/// in an `AccountTree`
/// 
trait ParentNode: AccountTreeNode + ParentNodeT {}


///
/// `ParentNode` implentation of the `AccountTreeNode` and `ParentNodeT` traits
/// 
impl<T> ParentNode for T where T: AccountTreeNode + ParentNodeT {}


///
/// `Debug` implementation for the `ParentNode` trait
/// 
impl Debug for dyn ParentNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{name: {:?}, level: {:?}}}", self.name(), self.level())
    }
}

///
/// A wrapper for a reference to a struct that implements the `ParentNode` trait
/// 
type ParentNodeRef = Rc<RefCell<dyn ParentNode>>;

///
/// A wrapper for a reference to a struct that implements the `AccountTreeNode` trait
/// 
type AccountTreeNodeRef = Rc<RefCell<dyn AccountTreeNode>>;


///
/// A wrapper for a reference to the `RootNode` struct
/// 
type RootNodeRef = Rc<RefCell<RootNode>>;

///
/// The top-level node of the Accounting Tree structure
/// 
struct RootNode {
    level: usize,
    name: String,
    parent: Option<ParentNodeRef>,
    children: Vec<ParentNodeRef>,
}

impl AccountTreeNode for RootNode {
    fn level(&self) -> usize {
        return self.level;
    }

    fn set_level(&mut self, level: usize) {
        _ = level
    }

    fn name(&self) -> &str {
        return self.name.as_str()
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned()
    }

    fn account_type(&self) -> &Option<Rc<PrimaryAccountType>> {
        return &None
    }

    // Used to set a child node's parent's
    fn set_parent(&mut self, parent: Option<Rc<RefCell<dyn ParentNode>>>) {
        _ = parent;
    }

    // Use to get the child node's parentOption
    fn parent(&self) -> &Option<Rc<RefCell<dyn ParentNode>>> {
        return &self.parent;
    }
}

impl ParentNodeT for RootNode {
    ///
    /// Add a child to the `AccountTagNode`
    /// 
    fn add_child(&mut self, child: ParentNodeRef) {
        self.children.push(child);
    }

    ///
    /// Get the children for this `AccountTagNode`
    /// 
    fn children(&self) -> &Vec<ParentNodeRef> {
        return &self.children;
    }
}

impl RootNode {
    fn new() -> Self {
        RootNode {
            level: 0, parent: None, name: "root".to_owned(), children:Vec::new(),
        }
    }
}

///
/// `AccountTag` structure used to define the category an account belongs to.
/// This node implements the `ChildNodeT`, `ParentNodeT` and `AccountTreeNode` traits.
/// This means that this node can be a parent or a child node or both on the tree.
///
struct AccountTagNode {
    level: usize,
    name: String,
    parent: Option<ParentNodeRef>,
    children: Vec<ParentNodeRef>,
    account_type: Option<Rc<PrimaryAccountType>>,
}

impl Debug for AccountTagNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AccountTagNode {{ level: {}, name: {}, parent: {:?}, account_type: {:?}}}, children: {:?}",
            self.level, self.name, self.parent, self.account_type, self.children)
    }
}

impl AccountTreeNode for AccountTagNode {
    fn level(&self) -> usize {
        return self.level;
    }

    fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    ///
    /// Get the name of the `AccountTagNode`
    /// 
    fn name(&self) -> &str {
        return self.name.as_str();
    }

    ///
    /// Set the name of the `AccountTagNode`
    /// 
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    ///
    /// Get the `PrimaryAccountType` of this tag node
    /// 
    fn account_type(&self) -> &Option<Rc<PrimaryAccountType>> {
        return &self.account_type;
    }

    // Used to set a child node's parent's
    fn set_parent(&mut self, parent: Option<Rc<RefCell<dyn ParentNode>>>) {
        _ = parent;
    }

    // Use to get the child node's parentOption
    fn parent(&self) -> &Option<Rc<RefCell<dyn ParentNode>>> {
        return &self.parent;
    }
}

///
/// `ParentNodeT` implementation for the `AccountTagNode`
/// 
impl ParentNodeT for AccountTagNode {
    ///
    /// Add a child to the `AccountTagNode`
    /// 
    fn add_child(&mut self, child: ParentNodeRef) {
        self.children.push(child);
    }

    ///
    /// Get the children for this `AccountTagNode`
    /// 
    fn children(&self) -> &Vec<ParentNodeRef> {
        return &self.children;
    }
}

impl AccountTagNode {
    fn new(level: usize, name: &str, parent: Option<Rc<RefCell<dyn ParentNode>>>, account_type: Option<Rc<PrimaryAccountType>>) -> Self {
        let children = Vec::new();
        let account_tag_node: AccountTagNode;

        // Panic if the level passed is less than one
        if level < 1 {
            panic!("Cannot have an AccountTagNode whose level {:?} is < 1.", level);
        } else if level > 1 {
            let mut _parent_ref = parent.clone().unwrap().clone();

            while _parent_ref.as_ref().borrow().level() != 1 {
                _parent_ref = {
                    // Borrow the parent reference
                    let parent_ref_borrowed = _parent_ref.as_ref().borrow();
                    // Get the parent
                    let parent = parent_ref_borrowed.parent().clone();
                    let parent_to_return: ParentNodeRef;
                    match parent {
                        None => { panic!("");},
                        Some(parent_unwrapped) => {
                            parent_to_return = parent_unwrapped.clone();
                        }
                    }

                    parent_to_return
                }
            }

            let tmp_acc_type = _parent_ref.as_ref().borrow();
            let tmp_acc_type = tmp_acc_type.account_type();

            account_tag_node = AccountTagNode{
                level, name: name.to_owned(), parent, children, account_type: tmp_acc_type.to_owned(),
            }
        } else {
            // Confirm that if the level == 1, an associated account_type exists
            match account_type {
                None => panic!("Level 1 nodes cannot miss an associated account type"),
                Some(account_type) => {
                    account_tag_node =  AccountTagNode{
                        level, name: name.to_owned(), parent, children, account_type: Some(account_type.clone()),
                    }
                }
            }
        }

        return account_tag_node;
    }
}

///
/// Node representing an actual account on the `AccountTree`.
/// This node only implements the `AccountTreeNode` and `ChildNodeT` traits as it can only be a terminal child node.
/// 
struct AccountNode {
    level: usize,
    name: String,
    amount: f64,
    parent: Option<ParentNodeRef>,
    children: Vec<ParentNodeRef>,
    account_type: Option<Rc<PrimaryAccountType>>,
}

impl Debug for AccountNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AccountTreeNode for AccountNode {
    fn level(&self) -> usize {
        return self.level
    }

    fn set_level(&mut self, level: usize) {
        self.level = level
    }

    ///
    /// Used to get the name of the `AccountNode`
    /// 
    fn name(&self) -> &str {
        return &self.name;
    }

    ///
    /// Used to set the name of the `AccountNode`
    /// 
    fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    ///
    ///  Used to set a child node's parent's
    /// 
    fn set_parent(&mut self, parent: Option<ParentNodeRef>) {
        _ = parent;
    }

    ///
    ///  Use to get the child node's parentOption
    /// 
    fn parent(&self) -> &Option<ParentNodeRef> {
        return &self.parent;
    }

    ///
    /// Get the `PrimaryAccountType` of this tag node
    /// 
    fn account_type(&self) -> &Option<Rc<PrimaryAccountType>> {
        return &self.account_type;
    }
}


impl ParentNodeT for AccountNode {
    fn add_child(&mut self, child: ParentNodeRef) {
        _ = child
    }

    fn children(&self) -> &Vec<ParentNodeRef> {
        return &self.children;
    }
}

impl AccountNode {
    fn new(level: usize, name: &str, amount: f64, parent: Option<Rc<RefCell<dyn ParentNode>>>) -> Self {
        // Get a clone of the parent
        let parent_ref = parent.clone().unwrap().clone();
        // Retrieve the account type of the parent and return it
        let parent_account_type = {
            let borrowed_ref = parent_ref.borrow();
            borrowed_ref.account_type().to_owned()
        };

        AccountNode{
            level, name: name.to_owned(), amount, parent, children: Vec::new(), account_type: parent_account_type,
        }
    }

    ///
    /// Used to set the amount in the `AccountNode`
    /// 
    fn set_amount(&mut self, amount: f64) {
        self.amount = amount
    }

    ///
    /// Used to get the amount in the `AccountNode`
    /// 
    fn amount(&self) -> f64 {
        return self.amount;
    }
}


///
/// `AccountTree` that holds the entire structure of the relation between different accounts.
/// It contains a single reference to the `RootNode` of the particular account structure
/// 
struct AccountTree {
    root: RootNode
}


///
/// `Ancestors` structure to get the parent of the current node upto the parent level
/// 
struct Ancestors{
    source:ParentNodeRef,
}

impl Ancestors {
    fn new(source: ParentNodeRef) -> Self {
        Ancestors {
            source
        }
    }

    ///
    /// Used to update the `source` of  the `Ancestors` struct
    /// 
    fn update_source(&mut self, source: ParentNodeRef) {
        self.source = source
    }
}

impl Iterator for Ancestors {
    type Item = ParentNodeRef;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.source.clone();
        let borrowed_node_parent = current_node.borrow();
        let node_parent = borrowed_node_parent.parent();
        match node_parent {
            None => { return None; },
            Some(parent) => {
                self.update_source(parent.clone());
            }
        }

        return Some(self.source.clone());
    }
}


///
/// `Descendants` structure used to get the children of the current node upto the leaves
/// 
struct Descendants {
    _source: ParentNodeRef,
    children:Vec<ParentNodeRef>,
}

impl Descendants {
    fn new(source: ParentNodeRef) -> Self {
        let children = source.as_ref().borrow().children().clone();

        Descendants {
            _source: source, children,
        }
    }

    ///
    /// Used to update the next descendants of this iterator
    /// 
    fn update_children(&mut self, children:Vec<ParentNodeRef>) {
        self.children = children;
    }

    ///
    /// Used to mark the `Descendants` Iterator as having consumed all the values
    /// 
    fn finished(&self) -> bool {
        return self.children.is_empty()
    }
}

impl Iterator for Descendants {
    type Item = Vec<ParentNodeRef>;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the list of the current children
        let children = self.children.clone();
        let mut next_children: Vec<ParentNodeRef> = Vec::new();

        // Append the children's children to the next_children vector.
        for child in &children {
            next_children.extend_from_slice(&child.as_ref().borrow().children().clone());
        }

        // Check if the loop is finished first and return None
        if self.finished() {
            return None;
        }

        self.update_children(next_children);

        return Some(children);
    }
}

fn main() {
    let asset: Rc<PrimaryAccountType> = Rc::new(
        PrimaryAccountType::new("Assets", ActionType::Increase, ActionType::Decrease)
    );
    let expense: Rc<PrimaryAccountType> = Rc::new(
        PrimaryAccountType::new("Expenses", ActionType::Increase, ActionType::Decrease)
    );

    let root: RootNodeRef = Rc::new(RefCell::new(RootNode::new()));

    let asset_node = Rc::new(
        RefCell::new(AccountTagNode::new(1, "Asset", Some(root.clone()), Some(asset.clone()))));
    
    {
        root.borrow_mut().add_child(asset_node.clone());
    }
    
    let current_assets_node = Rc::new(
        RefCell::new(AccountTagNode::new(2, "Current Assets",Some(asset_node.clone()), None))
    );


    // Necessary to drop the mutable borrowed reference
    {
        let mut asset_n = asset_node.borrow_mut();
        asset_n.add_child(current_assets_node.clone());
    }

    // An AccountNode's definition example
    let cash = Rc::new(
        RefCell::new(AccountNode::new(3, "Cash", 100_000.0, Some(current_assets_node.clone())))
    );

    {
        let mut current_asset_n = current_assets_node.borrow_mut();
        current_asset_n.add_child(cash.clone());

    }

    let cash_ref = cash.as_ref().borrow();
    let cash_acc_type = cash_ref.account_type();

    println!("Cash account type: {:?}", cash_acc_type);

    let node_ref = current_assets_node.as_ref().borrow();
    let acc_type = node_ref.account_type();
    println!("Retrieved account type is: {:?}", acc_type);


    // Get the descendants of the root node
    let mut descendants = Descendants::new(root.clone());

    println!("Next: {:?}", descendants.next());
    println!("Next: {:?}", descendants.next());
    println!("Next: {:?}", descendants.next());

    let mut ancestors = Ancestors::new(cash.clone());

    println!("**************************************");
    println!("Cash ancestors");
    println!("Ancestor: {:?}", ancestors.next());
    println!("Ancestor: {:?}", ancestors.next());
    println!("Ancestor: {:?}", ancestors.next());
}

