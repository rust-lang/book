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

function attachFerrises(type) {
  var elements = document.getElementsByClassName(type.attr)

  for (var codeBlock of elements) {
    var lines = codeBlock.innerText.replace(/\n$/, '').split(/\n/).length
    var size = 'large'
    if (lines < 4) {
      size = 'small'
    }

    var container = prepareFerrisContainer(codeBlock, size == 'small')
    container.appendChild(createFerris(type, size))
  }
}

function prepareFerrisContainer(element, useButtons) {
  var foundButtons = element.parentElement.querySelector('.buttons')
  if (useButtons && foundButtons) {
    return foundButtons
  }

  var div = document.createElement('div')
  div.classList.add('ferris-container')

  element.parentElement.insertBefore(div, element)

  return div
}

function createFerris(type, size) {
  var a = document.createElement('a')
  a.setAttribute('href', 'ch00-00-introduction.html#ferris')
  a.setAttribute('target', '_blank')

  var img = document.createElement('img')
  img.setAttribute('src', 'img/ferris/' + type.attr + '.svg')
  img.setAttribute('title', type.title)
  img.classList.add('ferris')
  img.classList.add('ferris-' + size)

  a.appendChild(img)

  return a
}
