const counter = document.querySelector(".counter__count")
const button = document.querySelector("button")
const state = document.querySelector(".state")

button.addEventListener("click", () => { 
  // connect to event source
  const eventSource = new EventSource('http://localhost:8080/progress')
  button.disabled = true
  setStateConnected()

  // update progress whenever there is new data
  eventSource.addEventListener('progress', () => {
    const data = JSON.parse(event.data) 
    counter.innerText = data.progress
    console.log(data)
  })

  // close the connection once the last (close) event comes
  eventSource.addEventListener('close', () => {
    eventSource.close()
    setStateDisconnected()
  })
})

// sets the state string to green and connected
const setStateConnected = () => {
  state.innerText = "Connected"
  state.classList.remove("state--disconnected")
  state.classList.add("state--connected")
}

// sets the state string to red and disconnected
const setStateDisconnected = () => {
  state.innerText = "Disconnected"
  state.classList.remove("state--connected")
  state.classList.add("state--disconnected")
}
