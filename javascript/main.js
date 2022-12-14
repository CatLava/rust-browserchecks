function apiCall(url, method) {
        let xhr = new XMLHttpRequest();
        xhr.withCredentials = true;
        xhr.open("POST", "/bfp/add");
        xhr.setRequestHeader('Content-Type', 'application/json');
        xhr.send(JSON.stringify({
        value: value })
        );
}
        