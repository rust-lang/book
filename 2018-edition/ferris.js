var ferrisTypes = [
  'does_not_compile',
  'panics',
  'unsafe',
  'not_desired_behavior',
]

document.addEventListener('DOMContentLoaded', () => {
  for (var ferrisType of ferrisTypes) {
    makeFerris(ferrisType)
  }
})

function makeFerris (type) {
  var elements = document.getElementsByClassName(type)

  for (var codeBlock of elements) {
    var img = document.createElement("img")
    img.setAttribute('src', 'img/ferris/' + type + '.svg')
    img.className = 'ferris'
    codeBlock.parentElement.insertBefore(img, codeBlock)
  }
}
