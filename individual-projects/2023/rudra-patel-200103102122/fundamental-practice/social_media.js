// step 1: check the social media (insta,facebook,mail)
// step 2: see the post is relevant to you then stay in that page or post or else scroll
//         if you like the post then like, share or Comment.

// step 3: if mobile network is not available then show error. or while network comes wait for it.
// step 4: give login function.
// step 5: requst function if person is appropreate than approve it otherwise decline it or select all the request and decline it.

function post() {
    let releveent_post = true
    let network = true
    for (let i = 0; i <= 10; i++) {
        if (releveent_post) {
            console.log('stay in that page or post & like, share or Comment')
        } else {
            console.log('scroll')
        }
        if (i == 10) {
            console.log('add some more post')
            post_repet()
        }
        while (!network) {
            console.log('page not found')
        }
    }
}

function post_repet() {
    for (let i = 0; i < 10; i++) {
        post()
    }
}

function login(insta, facebook, mail) {
    const user_name = prompt('Enter your user_name: ')
    const password = prompt('enter your pasword: ')
    post()
}

function friend_request(insta, facebook) {
    let followers = 250;
    let is_request = "accept";
    for (let i = 0; i <= 10; i++) {
        if (requst === "accept") {
            console.log('you want to approve than accept')
            followers++;
        }
        if (is_request === "selectall"){
                console.log('select all the request and decline it')
        }
        if (requst === "decline") {
           console.log('decline your request')
       }
        else {
            
        }
    }
}

function main() {
    let social_media = 'insta'
    switch (social_media) {
        case 'insta':
            console.log('acessing insta')
            login(insta, facebook, mail)
            friend_request(insta, facebook)
            break
        case 'facebook':
            console.log('acessing facebook')
            login(insta, facebook, mail)
            friend_request(insta, facebook)
            break
        case 'mail':
            console.log('acessing mail')
            login(insta, facebook, mail)
            break
        default:
            console.log('this is not a social media')
            break
    }
}
