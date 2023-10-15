<script>
    /**
   * @type {any}
   */
     export let completed
     /**
   * @type {any}
   */
      export let title
     /**
   * @type {any}
   */
      export let description
</script>

<div class="card w-96 bg-base-100 shadow-xl image-full">
    <figure><img src="/images/stock/photo-1606107557195-0e29a4b5b4aa.jpg" alt="Shoes" /></figure>
  <div class="card-body">
    {#if completed}
 <svg class="z-10 absolute top-6 right-6" fill="gold" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z"/></svg> 
 {/if}
    <h2 class="card-title">{title}</h2>
    <p>{description}</p>
    <div class="card-actions justify-end">
      <button class="btn btn-primary">Buy Now</button>
    </div>
  </div>
</div>