// Instagram
// Scroll through posts.
// If post is relevant then like share comment.

// accept all request.
// reject all request.
// accept/ reject requests one by one
// Handle network connectivity. Wait until network is back

fn scroll(){

}
fn relevent(){
     return true;
}
fn request(){
     
}
fn accept(){
     return true;
}
fn reject(){
     return true;
}
fn network(){
     return true;
}
 
fn main(){
    for i in scroll(){
        if relevent(){
            println!("like,share,comment");
        }

    }

    for i in request(){
        if accept(){
            println!("accept all request");
        }
        else if reject(){
            println!("reject all request");
        }
        while !network(){
            println!("wait until network is back");
        }
    }
}



  