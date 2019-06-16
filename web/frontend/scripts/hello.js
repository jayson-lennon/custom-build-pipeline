class Req {
  static get(url) {
    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest();
      xhr.open("GET", url);
      xhr.onload = () => resolve(xhr.responseText);
      xhr.onerror = () => reject(xhr.statusText);
      xhr.send();
    });
  }
}

let el = document.getElementById("greet");

Req.get("http://localhost:8080/{%USERNAME%}")
  .then((data) => {
    el.innerText = data;
  })
  .catch((err) => {
    el.innerText = "Server error";
  });

