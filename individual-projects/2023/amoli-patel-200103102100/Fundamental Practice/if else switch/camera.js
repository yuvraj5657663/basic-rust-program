//Write a code to validate cameras of basement parking is working or not. Check all 3 basements camera and print the camera ids if not working. If 1 camera from 3 is not working then show warning, if more than 1 is not working then show error

function main() {
    let working = 0,
        not_working = 0,
        basement = 3,
        camera_id = ['a1', 'a2', 'a3', 'b1', 'b2', 'b3', 'c1', 'c2', 'c3']

    for (n = 0; n < camera_id.length; n += basement) {
        const basements = camera_id.slice(n, n + basement)
        console.log('basement', basements)
    }
    for (m = 0; m < camera_id.length; m++) {
        if (m === 1 || m === 5 || m === 6 || m === 2) {
            console.log('not working camera id:', camera_id[m])
            not_working++
        } else {
            working++
        }
    }
    if (not_working === 1) {
        console.log('warning 1 camera not working')
    } else if (not_working > 1) {
        console.log('error more than one camera not working')
    } else {
        console.log('all cameras are working')
    }
}
main()
