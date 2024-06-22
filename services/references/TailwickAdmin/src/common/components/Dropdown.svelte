<script>
    import { getContext, setContext, onDestroy } from 'svelte';
    import { createPopperActions } from './Popper'; 
    import { createContext } from './DropdownContext';
  
    const noop = () => undefined;
  
    let context = createContext();
    setContext('dropdownContext', context);
    const navbarContext = getContext('navbar');
  
    export let className='';
    export let autoClose = true;
    export let direction = 'bottom-start';
    export let isOpen = false;
    export let setActiveFromChild = false;
    export let toggle = undefined;

  const [popperRef, popperContent] = createPopperActions();
  
    let component;
    let dropdownDirection;
  
    $: subItemIsActive = !!(
      setActiveFromChild &&
      component &&
      typeof component.querySelector === 'function' &&
      component.querySelector('.active')
    );
  
    $: {
      if (direction === 'left') dropdownDirection = 'start';
      else if (direction === 'right') dropdownDirection = 'end';
      else dropdownDirection = direction;
    }
    $: {
      if (typeof document !== 'undefined') {
        if (isOpen) {
          ['click', 'touchstart', 'keyup'].forEach((event) =>
            document.addEventListener(event, handleDocumentClick, true)
          );
        } else {
          ['click', 'touchstart', 'keyup'].forEach((event) =>
            document.removeEventListener(event, handleDocumentClick, true)
          );
        }
      }
    }
  
    $: {
      context.update(() => {
        return {
          toggle: handleToggle,
          isOpen,
          autoClose,
          direction: direction,
          popperRef: popperRef,
          popperContent: popperContent,
        };
      });
    }
  
    $: handleToggle = toggle || (() => (isOpen = !isOpen));
  
    function handleDocumentClick(e) {
      if (e && (e.which === 3 || (e.type === 'keyup' && e.which !== 9))) return;
  
      if (
        component.contains(e.target) &&
        component !== e.target &&
        (e.type !== 'keyup' || e.which === 9)
      ) {
        return;
      }
  
      if (autoClose === true || autoClose === 'inside') {
        handleToggle(e);
      }
    }
  
    onDestroy(() => {
      if (typeof document !== 'undefined') {
        ['click', 'touchstart', 'keyup'].forEach((event) =>
          document.removeEventListener(event, handleDocumentClick, true)
        );
      }
    });
  </script>
  
    <div  class='dropdown {className}' bind:this={component} >
      <slot />
    </div>