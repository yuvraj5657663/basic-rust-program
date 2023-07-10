// step 1: check the social media (insta,facebook,mail)
// step 2: see the post is relevant to you then stay in that page or post or else scroll
//         if you like the post then like, share or Comment.
// step 3: if mobile network is not available then show error and continue when network is available.
// step 4: login information
// step 5: if request seems appropriate then approve else decline
// step 6: approve or decline all together

function post() {
    let relevent, liked
    for (page = 0; page < 5; page++) {
        for (post = 0; post < 10; post++) {
            if (relevent == true) {
                if (relevent == liked) {
                    console.log('like,share,comment')
                }
            }
            while (network() == false) {
                console.log('network error')
            }
        }
    }
}
function request() {
    let friend_request, interest, multiselection
    while (friend_request == true) {
        console.log('you have requests')
        if (multiselection == true) {
            for (select_all = 0; select_all < 10; select_all++) {
                if (interest) {
                    console.log('accept all together')
                } else {
                    console.log('decline all together')
                }
            }
        } else {
            if (interest) {
                console.log('accept')
            } else {
                console.log('decline')
            }
        }
        friend_request++
    }
}
function network() {
    let network
    if (network == true) {
        return true
    } else {
        return false
    }
}
function login(instagram, facebook, mail) {
    if (instagram == true) {
        console.log('open instagram')
    } else if (facebook == true) {
        console.log('open facebook')
    } else if (mail == true) {
        console.log('open mail')
    }
    let username = prompt('Enter username')
    let password = prompt('Enter password')
    request()
    post()
}
function main() {
    let media = ['instagram', 'facebook', 'mail']
    switch (media) {
        case 'instagram':
        case 'facebook':
        case 'mail': {
            console.log('its a media')
            login(instagram, facebook, mail)
            break
        }
        default: {
            console.log('not a media')
        }
    }
}
main()
