use std::{cell::RefCell, fmt::Debug, rc::Rc};

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
    fn add_child(&mut self, child: Rc<RefCell<dyn AccountTreeNode>>);

    // Used to get the children of the parent node
    fn children(&self) -> &Vec<Rc<RefCell<dyn AccountTreeNode>>>;
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
/// `ChildNode` trait that marks a node as being able to have a `Parent` which
/// it falls under
/// 
// trait ChildNode: AccountTreeNode + ChildNodeT {}


// ///
// /// `ChildNode` implementation of the `AccountTreeNode` and `ChildNodeT` traits
// /// 
// impl<T> ChildNode for T where T: AccountTreeNode + ChildNodeT {}

// impl Debug for dyn ChildNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{name: {:?}, level: {:?}, parent: {:?}}}", self.name(), self.level(), self.parent());
//     }
// }

///
/// The top-level node of the Accounting Tree structure
/// 
struct RootNode {
    level: usize,
    name: String,
    parent: Option<Rc<RefCell<dyn ParentNode>>>,
    children: Vec<Rc<RefCell<dyn AccountTreeNode>>>,
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
    fn add_child(&mut self, child: Rc<RefCell<dyn AccountTreeNode>>) {
        self.children.push(child);
    }

    ///
    /// Get the children for this `AccountTagNode`
    /// 
    fn children(&self) -> &Vec<Rc<RefCell<dyn AccountTreeNode>>> {
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
    parent: Option<Rc<RefCell<dyn ParentNode>>>,
    children: Vec<Rc<RefCell<dyn AccountTreeNode>>>,
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
        println!("{:?}", self.level);
        // Root nodes and level-1 nodes have a direct access to their account types
        if self.level <= 1 {
            return &self.account_type
        }

        let mut parent_node = self.parent.as_ref();

        while parent_node.unwrap().borrow().level() != 1 {
            match parent_node {
                None => { return &None; },
                Some(p_node) => {
                    parent_node = Some(p_node);
                }
            }
        }
        let res =  parent_node.unwrap().borrow();
        let res = res.account_type();
        println!("AccountType: {:?}", res.as_ref().unwrap());
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
/// `ChildNodeT` implementation for the `AccountTagNode`
/// 
// impl ChildNodeT for AccountTagNode {
//     fn parent(&self) -> &RefCell<dyn ParentNode> {
//         return self.parent.as_ref();
//     }

//     fn set_parent(&mut self, parent: Rc<RefCell<dyn ParentNode>>) {
//         self.parent = parent;
//     }
// }


///
/// `ParentNodeT` implementation for the `AccountTagNode`
/// 
impl ParentNodeT for AccountTagNode {
    ///
    /// Add a child to the `AccountTagNode`
    /// 
    fn add_child(&mut self, child: Rc<RefCell<dyn AccountTreeNode>>) {
        self.children.push(child);
    }

    ///
    /// Get the children for this `AccountTagNode`
    /// 
    fn children(&self) -> &Vec<Rc<RefCell<dyn AccountTreeNode>>> {
        return &self.children;
    }
}

impl AccountTagNode {
    fn new(level: usize, name: &str, parent: Option<Rc<RefCell<dyn ParentNode>>>, account_type: Option<Rc<PrimaryAccountType>>) -> Self {
        let children = Vec::new();
        let account_tag_node: AccountTagNode;

        // Add this as a child to the passed parent node
        if level != 1 {
            account_tag_node = AccountTagNode{
                level, name: name.to_owned(), parent, children, account_type: None,
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

    ///
    /// Set the `PrimaryAccountType` of this tag node
    /// Only level 1 account nodes can get PrimaryAccoutType set.
    /// 
    fn set_account_type(&mut self, account_type: Rc<PrimaryAccountType>) {
        if self.level == 1 {
            self.account_type = Some(account_type.clone());
        }
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
    parent: Option<Rc<RefCell<dyn ParentNode>>>
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
    fn set_parent(&mut self, parent: Option<Rc<RefCell<dyn ParentNode>>>) {
        _ = parent;
    }

    ///
    ///  Use to get the child node's parentOption
    /// 
    fn parent(&self) -> &Option<Rc<RefCell<dyn ParentNode>>> {
        return &self.parent;
    }

    ///
    /// Get the `PrimaryAccountType` of this tag node
    /// 
    fn account_type(&self) -> &Option<Rc<PrimaryAccountType>> {
        return &None
    }
}

///
/// `ChildNodeT` implementation for the `AccountNode`
/// 
// impl ChildNodeT for AccountNode {
//     fn parent(&self) -> &RefCell<dyn ParentNode> {
//         return self.parent.as_ref()
//     }

//     fn set_parent(&mut self, parent: Rc<RefCell<dyn ParentNode>>) {
//         self.parent = parent;
//     }
// }

impl AccountNode {
    fn new(level: usize, name: &str, amount: f64, parent: Option<Rc<RefCell<dyn ParentNode>>>) -> Self {
        AccountNode{
            level, name: name.to_owned(), amount, parent
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

#[derive(Debug)]
struct Account {
    name: String,
    account_tag: Rc<AccountTagNode>,
}

impl Account {
    fn new(name: &str, account_tag: Rc<AccountTagNode>) -> Self {
        Account {
            name: name.to_owned(), account_tag
        }
    }

    // Get the account name
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn account_type(&self) -> &AccountTagNode{
        self.account_tag.as_ref()
    }

    pub fn set_name(&mut self, name: String){
        self.name = name
    }

    pub fn set_account_type(&mut self, account_tag: Rc<AccountTagNode>){
        self.account_tag = account_tag
    }

    ///
    /// Get the `PrimaryAccountType` of this account
    /// 
    fn primary_account(&self) -> &Option<PrimaryAccountType> {
        let mut parent_tag= self.account_tag.as_ref();

        while parent_tag.level() != 1 {
            let parent_tag = parent_tag.parent();
        }
        
        return &None;
    }
}

fn main() {
    let asset: Rc<PrimaryAccountType> = Rc::new(
        PrimaryAccountType::new("Assets", ActionType::Increase, ActionType::Decrease)
    );
    let expense: Rc<PrimaryAccountType> = Rc::new(
        PrimaryAccountType::new("Expenses", ActionType::Increase, ActionType::Decrease)
    );
    let root: Rc<RefCell<RootNode>> = Rc::new(RefCell::new(RootNode::new()));

    let asset_node = Rc::new(
        RefCell::new(AccountTagNode::new(1, "Asset", Some(root.clone()), Some(asset.clone()))));
    
    let current_assets_node = Rc::new(
        RefCell::new(AccountTagNode::new(2, "Current Assets",Some(asset_node.clone()), None))
    );

    // Necessary to drop the mutable borrowed reference
    {
        let mut asset_n = asset_node.as_ref().borrow_mut();
        asset_n.add_child(current_assets_node.clone());
    }

    // An AccountNode's definition example
    let cash = Rc::new(
        RefCell::new(AccountNode::new(3, "Cash", 100_000.0, Some(current_assets_node.clone())))
    );

    {
        let mut current_asset_n = current_assets_node.as_ref().borrow_mut();
        current_asset_n.add_child(cash.clone());

    }

    // println!("Current Asset node: {:?}", asset_node.clone());
    current_assets_node.borrow().account_type();
}

