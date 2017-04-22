$(document).ready(function(){

  $.get('http://rustpe.org/events', function(data){
    $.each(data, function(k, v) {
      $('#events').append('<h3>'+v.name+'</h3><h4>'+convertDate(v.date)+'</h4><p>'+v.about+'</p>');
    })
  });

  function convertDate(date){
    return moment(date).format('DD/MM/YYYY hh:mm A');
  }

});
