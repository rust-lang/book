var ferrisTypes = [
  {
    attr: 'does_not_compile',
    title: 'This code does not compile!'
  },
  {
    attr: 'panics',
    title: 'This code panics!'
  },
  {
    attr: 'not_desired_behavior',
    title: 'This code does not produce the desired behavior.'
  }
]

document.addEventListener('DOMContentLoaded', () => {
  for (var ferrisType of ferrisTypes) {
    attachFerrises(ferrisType)
  }
})

function attachFerrises (type) {
  var elements = document.getElementsByClassName(type.attr)

  for (var codeBlock of elements) {
    var lines = 0;
    for (var child of codeBlock.childNodes) {
      if(child.nodeType ===  Node.ELEMENT_NODE && child.classList.contains('boring')) {
        continue;
      }

      var text = child.textContent;
      if(/\r|\r\n|\n/.exec(text)) {
        lines += text.split(/\r|\r\n|\n/).length -1;
      }
    }

    attachFerris(codeBlock, type, lines)
  }
}

function attachFerris (element, type, lines) {
  var a = document.createElement('a')
  a.setAttribute('href', 'ch00-00-introduction.html#ferris')
  a.setAttribute('target', '_blank')

  var img = document.createElement('img')
  img.setAttribute('src', 'img/ferris/' + type.attr + '.svg')
  img.setAttribute('title', type.title)
  img.className = 'ferris'

  if(lines < 4) {
    img.style.width = "30px"
    img.style.height = "30px"
  }

  a.appendChild(img)

  element.parentElement.insertBefore(a, element)
}
